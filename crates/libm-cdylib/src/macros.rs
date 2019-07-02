macro_rules! export {
    (fn $id:ident ($($arg:ident : $arg_ty:ty),* ) -> $ret_ty:ty:
       ($($test_arg:expr),*) -> $test_ret:expr;) => {
        #[no_mangle]
        pub extern "C" fn $id($($arg: $arg_ty),*) -> $ret_ty {
            // This just forwards the call to the appropriate function of the
            // libm crate.
            #[cfg(not(link_test))] {
                libm::$id($($arg),*)
            }
            // When generating the linking tests, we return a specific incorrect
            // value. This lets us tell this libm from the system's one appart:
            #[cfg(link_test)] {
                // TODO: as part of the rountrip, we probably want to verify
                // that the argument values are the unique ones provided.
                let _ = libm::$id($($arg),*);
                $test_ret as _
            }
        }

        #[cfg(test)]
        paste::item! {
            // These tests check that the library links properly.
            #[test]
            fn [<$id _link_test>]() {
                use crate::test_utils::*;

                // This re-compiles the dynamic library:
                compile_cdylib();

                // Generate a small C program that calls the C API from
                // <math.h>. This program prints the result into an appropriate
                // type, that is then printed to stdout.
                let (cret_t, c_format_s)
                    = ctype_and_printf_format_specifier(stringify!($ret_ty));
                let ctest = format!(
                    r#"
                        #include <math.h>
                        #include <stdio.h>
                        #include <stdint.h>
                        int main() {{
                            {cret_t} result = {id}({input});
                            fprintf(stdout, "{c_format_s}", result);
                            return 0;
                        }}
                    "#,
                    id = stringify!($id),
                    input = [$(stringify!($test_arg)),*].join(","),
                    cret_t = cret_t,
                    c_format_s = c_format_s
                );

                let target_dir = target_dir();
                eprintln!("target dir: {}", target_dir.display());
                let src_path = target_dir.clone().join(format!("{}.c", stringify!($id)));
                let bin_path = target_dir.clone().join(format!("{}", stringify!($id)));
                write_to_file(&src_path, &ctest);

                // We now compile the C program into an executable, make sure
                // that the libm-cdylib has been generated (and generate it if
                // it isn't), and then we run the program, override the libm,
                // and verify the result.
                compile_file(&src_path, &bin_path);
                check(&bin_path, $test_ret as $ret_ty)
            }
        }
    };
     ($(fn $id:ident ($($arg:ident : $arg_ty:ty),* ) -> $ret_ty:ty:
        ($($test_arg:expr),*) -> $test_ret:expr;)*) => {
        $(
            export! { fn $id ($($arg : $arg_ty),* ) -> $ret_ty: ($($test_arg),*) -> $test_ret; }
        )*
    }
}
