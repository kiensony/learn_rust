use task_1652::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_total(""), 0);
    assert_eq!(major_version_total("0.0.1"), 0);
}
