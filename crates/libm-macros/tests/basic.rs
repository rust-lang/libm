libm_macros::for_each_function! {
    callback: foo,
    skip: [foo, bar],
    attributes: [
        #[meta1]
        #[meta2]
        [baz, corge]
    ],
}
