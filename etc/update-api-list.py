#!/usr/bin/env python3
"""Create a text file listing all public API. This can be used to ensure that all
functions are covered by our macros.
"""

import json
import subprocess as sp
import sys
import difflib
from pathlib import Path
from typing import Any

ETC_DIR = Path(__file__).parent


def get_rustdoc_json() -> dict[Any, Any]:
    """Get rustdoc's JSON output for the `libm` crate."""

    librs_path = ETC_DIR.joinpath("../src/lib.rs")
    j = sp.check_output(
        [
            "rustdoc",
            librs_path,
            "--edition=2021",
            "--output-format=json",
            "-Zunstable-options",
            "-o-",
        ],
        text=True,
    )
    j = json.loads(j)
    return j


def list_public_functions() -> list[str]:
    """Get a list of public functions from rustdoc JSON output.

    Note that this only finds functions that are reexported in `lib.rs`, this will
    need to be adjusted if we need to account for functions that are defined there.
    """
    names = []
    index: dict[str, dict[str, Any]] = get_rustdoc_json()["index"]
    for item in index.values():
        # Find public items
        if item["visibility"] != "public":
            continue

        # Find only reexports
        if "use" not in item["inner"].keys():
            continue

        # Locate the item that is reexported
        id = item["inner"]["use"]["id"]
        srcitem = index.get(str(id))

        # External crate
        if srcitem is None:
            continue

        # Skip if not a function
        if "function" not in srcitem["inner"].keys():
            continue

        names.append(srcitem["name"])

    names.sort()
    return names


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

    names = list_public_functions()
    output = "# autogenerated by update-api-list.py\n"
    for name in names:
        output += f"{name}\n"

    out_file = ETC_DIR.joinpath("function-list.txt")

    if check:
        with open(out_file, "r") as f:
            current = f.read()
        diff_and_exit(current, output)
    else:
        with open(out_file, "w") as f:
            f.write(output)


if __name__ == "__main__":
    main()