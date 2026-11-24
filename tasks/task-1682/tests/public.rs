use task_1682::*;

#[test]
fn public_examples() {
    assert_eq!(preview_version_total(""), 0);
    assert_eq!(preview_version_total("0.0.1"), 1);
}
