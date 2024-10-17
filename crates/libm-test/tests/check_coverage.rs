//! Ensure that `for_each_function!` isn't missing any symbols.

/// Files in `src/` that do not export a testable symbol.
const ALLOWED_SKIPS: &[&str] = &[
    // Not a generic test function
    "fenv",
    // Nonpublic functions
    "expo2",
    "k_cos",
    "k_cosf",
    "k_expo2",
    "k_expo2f",
    "k_sin",
    "k_sinf",
    "k_tan",
    "k_tanf",
    "rem_pio2",
    "rem_pio2_large",
    "rem_pio2f",
];

macro_rules! function_names {
    (
        @all_items
        fn_names: [ $( $name:ident ),* ]
    ) => {
        const INCLUDED_FUNCTIONS: &[&str] = &[ $( stringify!($name) ),* ];
    };
    (@each_signature $($tt:tt)*) => {};
}

libm::for_each_function!(function_names);

#[test]
fn test_for_each_function_all_included() {
    let mut missing = Vec::new();

    for f in libm_test::ALL_FUNCTIONS {
        if !INCLUDED_FUNCTIONS.contains(f) && !ALLOWED_SKIPS.contains(f) {
            missing.push(f)
        }
    }

    if !missing.is_empty() {
        panic!(
            "missing tests for the following: {missing:#?} \
            \nmake sure any new functions are entered in the \
            `for_each_function` macro definition."
        );
    }
}
