use task_1662::*;

#[test]
fn public_examples() {
    assert_eq!(minor_version_total(""), 0);
    assert_eq!(minor_version_total("0.0.1"), 0);
}
