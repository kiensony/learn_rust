use task_278::*;

#[test]
fn public_examples() {
    assert_eq!(normalize_spaces_star("cat"), "cat");
    assert_eq!(normalize_spaces_star("*hi*"), "*hi*");
}
