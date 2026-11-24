use task_270::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_prefix_star("cat"), "cat");
    assert_eq!(remove_a_prefix_star("*hi*"), "hi*");
}
