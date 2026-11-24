use task_271::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_suffix_star("cat"), "cat");
    assert_eq!(remove_a_suffix_star("*hi*"), "*hi");
}
