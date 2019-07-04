# libm-test

This crate contains different types of test for the `libm` crate.

* `tests/system.rs`: generate random inputs, and tests that the results of the
  `libm` crate are within the tolerance required by the IEEE from those of the
  system's libm library (e.g. musl, glibc's libm, libSystem_m, etc.).
  
* `tests/unit.rs`: contains some small unit tests. 
