use task_292::*;

#[test]
fn public_examples() {
    assert_eq!(boundary_trim_plus("cat"), "cat");
    assert_eq!(boundary_trim_plus("+hi+"), "hi");
}
