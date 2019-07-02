macro_rules! export {
    (fn $id:ident ($($arg:ident : $arg_ty:ty),* ) -> $ret_ty:ty:
       ($($test_arg:expr),*) -> $test_ret:expr;) => {
        #[no_mangle]
        pub extern "C" fn $id($($arg: $arg_ty),*) -> $ret_ty {
            #[cfg(link_test)] {
                let _ = libm::$id($($arg),*);
                $test_ret as _
            }
            #[cfg(not(link_test))] {
                libm::$id($($arg),*)
            }
        }

        #[cfg(test)]
        paste::item! {
            #[test]
            fn [<$id _test>]() {
                use crate::test_utils::*;
                let (cret_t, c_format_s) = ctype_and_cformat(stringify!($ret_ty));
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

                let src = &format!("../../target/{}.c", stringify!($id));
                let bin = &format!("../../target/{}", stringify!($id));
                let src_path = std::path::Path::new(src);
                let bin_path = std::path::Path::new(bin);
                write_to_file(&src_path, &ctest);
                compile_file(&src_path, &bin_path);
                compile_lib();
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
