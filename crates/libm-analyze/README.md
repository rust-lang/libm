# libm-analyze

This crate provides a single macro, `for_each_api`:

```rust
macro_rules! nop {
    (
        id: $id:ident;
        arg_tys: $($arg_tys:ty),*;
        arg_ids: $($arg_ids:ident),*;
        ret: $ty:ty;
    ) => {};
}

libm_analyze::for_each_api!(nop);
```

This macro takes a user-provided macro, and expands it for all libm APIs.

For example, see how the `libm-test` crate `tests/system.rs` test uses it to
test all `libm` APIs against random inputs, and verify the results against the
system's libm library.
