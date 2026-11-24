use task_290::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_prefix_plus("cat"), "cat");
    assert_eq!(remove_a_prefix_plus("+hi+"), "hi+");
}
