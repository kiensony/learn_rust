mod common;

#[test]
fn examples() {
    common::check(&["a", "b"], "arguments=2\n", "");
    common::check(&["two words"], "arguments=1\n", "");
}
