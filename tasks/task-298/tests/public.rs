use task_298::*;

#[test]
fn public_examples() {
    assert_eq!(normalize_spaces_plus("cat"), "cat");
    assert_eq!(normalize_spaces_plus("+hi+"), "+hi+");
}
