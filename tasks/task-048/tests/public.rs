mod common;

#[test]
fn examples() {
    common::check(&[], "License: MIT\n", "");
    common::check(&["unused"], "License: MIT\n", "");
}
