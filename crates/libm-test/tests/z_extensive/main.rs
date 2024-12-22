#[cfg(not(feature = "test-multiprecision"))]
fn main() {}

#[cfg(feature = "test-multiprecision")]
mod run;

#[cfg(feature = "test-multiprecision")]
fn main() {
    run::run();
}
