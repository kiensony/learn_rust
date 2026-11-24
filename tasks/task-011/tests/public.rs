mod common;

#[test]
fn examples() {
    common::check(&[], "Hello, Rust!\n", "");
    common::check(&["unused"], "Hello, Rust!\n", "");
}
