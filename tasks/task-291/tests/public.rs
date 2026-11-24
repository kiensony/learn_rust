use task_291::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_suffix_plus("cat"), "cat");
    assert_eq!(remove_a_suffix_plus("+hi+"), "+hi");
}
