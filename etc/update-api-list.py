#!/usr/bin/env python3
"""Create a text file listing all public API. This can be used to ensure that all
functions are covered by our macros.
"""

import difflib
import json
import subprocess as sp
import sys
from dataclasses import dataclass
from glob import glob
from pathlib import Path
from typing import Any, TypeAlias

ETC_DIR = Path(__file__).parent
ROOT_DIR = ETC_DIR.parent

IndexTy: TypeAlias = dict[str, dict[str, Any]]
"""Type of the `index` item in rustdoc's JSON output"""


@dataclass
class Crate:
    """Representation of public interfaces and function defintion locations in
    `libm`.
    """

    public_functions: list[str]
    """List of all public functions."""
    defs: dict[str, list[str]]
    """Map from `name->[source files]` to find all places that define a public
    function. We track this to know which tests need to be rerun when specific files
    get updated.
    """
    types: dict[str, str]
    """Map from `name->type`."""

    def __init__(self) -> None:
        self.public_functions = []
        self.defs = {}
        self.types = {}

        j = self.get_rustdoc_json()
        index: IndexTy = j["index"]
        self._init_function_list(index)
        self._init_defs(index)
        self._init_types()

    @staticmethod
    def get_rustdoc_json() -> dict[Any, Any]:
        """Get rustdoc's JSON output for the `libm` crate."""

        j = sp.check_output(
            [
                "rustdoc",
                "src/lib.rs",
                "--edition=2021",
                "--document-private-items",
                "--output-format=json",
                "--cfg=f16_enabled",
                "--cfg=f128_enabled",
                "-Zunstable-options",
                "-o-",
            ],
            cwd=ROOT_DIR,
            text=True,
        )
        j = json.loads(j)
        return j

    def _init_function_list(self, index: IndexTy) -> None:
        """Get a list of public functions from rustdoc JSON output.

        Note that this only finds functions that are reexported in `lib.rs`, this will
        need to be adjusted if we need to account for functions that are defined there, or
        glob reexports in other locations.
        """
        # Filter out items that are not public
        public = [i for i in index.values() if i["visibility"] == "public"]

        # Collect a list of source IDs for reexported items in `lib.rs` or `mod math`.
        use = (i for i in public if "use" in i["inner"])
        use = (
            i for i in use if i["span"]["filename"] in ["src/math/mod.rs", "src/lib.rs"]
        )
        reexported_ids = [item["inner"]["use"]["id"] for item in use]

        # Collect a list of reexported items that are functions
        for id in reexported_ids:
            srcitem = index.get(str(id))
            # External crate
            if srcitem is None:
                continue

            # Skip if not a function
            if "function" not in srcitem["inner"]:
                continue

            self.public_functions.append(srcitem["name"])
        self.public_functions.sort()

    def _init_defs(self, index: IndexTy) -> None:
        defs = {name: set() for name in self.public_functions}
        funcs = (i for i in index.values() if "function" in i["inner"])
        funcs = (f for f in funcs if f["name"] in self.public_functions)
        for func in funcs:
            defs[func["name"]].add(func["span"]["filename"])

        # A lot of the `arch` module is often configured out so doesn't show up in docs. Use
        # string matching as a fallback.
        for fname in glob("src/math/arch/**.rs", root_dir=ROOT_DIR):
            contents = (ROOT_DIR.joinpath(fname)).read_text()

            for name in self.public_functions:
                if f"fn {name}" in contents:
                    defs[name].add(fname)

        for name, sources in defs.items():
            base_sources = defs[base_name(name)[0]]
            for src in (s for s in base_sources if "generic" in s):
                sources.add(src)

        # Sort the set
        self.defs = {k: sorted(v) for (k, v) in defs.items()}

    def _init_types(self) -> None:
        self.types = {name: base_name(name)[1] for name in self.public_functions}

    def write_function_list(self, check: bool) -> None:
        """Collect the list of public functions to a simple text file."""
        output = "# autogenerated by update-api-list.py\n"
        for name in self.public_functions:
            output += f"{name}\n"

        out_file = ETC_DIR.joinpath("function-list.txt")

        if check:
            with open(out_file, "r") as f:
                current = f.read()
            diff_and_exit(current, output)
        else:
            with open(out_file, "w") as f:
                f.write(output)

    def write_function_defs(self, check: bool) -> None:
        """Collect the list of information about public functions to a JSON file ."""
        comment = (
            "Autogenerated by update-api-list.py. "
            "List of files that define a function with a given name. "
            "This file is checked in to make it obvious if refactoring breaks things"
        )

        d = {"__comment": comment}
        d |= {
            name: {"sources": self.defs[name], "type": self.types[name]}
            for name in self.public_functions
        }

        out_file = ETC_DIR.joinpath("function-definitions.json")
        output = json.dumps(d, indent=4) + "\n"

        if check:
            with open(out_file, "r") as f:
                current = f.read()
            diff_and_exit(current, output)
        else:
            with open(out_file, "w") as f:
                f.write(output)


def diff_and_exit(actual: str, expected: str):
    """If the two strings are different, print a diff between them and then exit
    with an error.
    """
    if actual == expected:
        print("output matches expected; success")
        return

    a = [f"{line}\n" for line in actual.splitlines()]
    b = [f"{line}\n" for line in expected.splitlines()]

    diff = difflib.unified_diff(a, b, "actual", "expected")
    sys.stdout.writelines(diff)
    print("mismatched function list")
    exit(1)


def base_name(name: str) -> tuple[str, str]:
    """Return the basename and type from a full function name. Keep in sync with Rust's
    `fn base_name`.
    """
    known_mappings = [
        ("erff", ("erf", "f32")),
        ("erf", ("erf", "f64")),
        ("modff", ("modf", "f32")),
        ("modf", ("modf", "f64")),
        ("lgammaf_r", ("lgamma_r", "f32")),
        ("lgamma_r", ("lgamma_r", "f64")),
    ]

    found = next((base for (full, base) in known_mappings if full == name), None)
    if found is not None:
        return found

    if name.endswith("f"):
        return (name.rstrip("f"), "f32")

    if name.endswith("f16"):
        return (name.rstrip("f16"), "f16")

    if name.endswith("f128"):
        return (name.rstrip("f128"), "f128")

    return (name, "f64")


def main():
    """By default overwrite the file. If `--check` is passed, print a diff instead and
    error if the files are different.
    """
    match sys.argv:
        case [_]:
            check = False
        case [_, "--check"]:
            check = True
        case _:
            print("unrecognized arguments")
            exit(1)

    crate = Crate()
    crate.write_function_list(check)
    crate.write_function_defs(check)


if __name__ == "__main__":
    main()
