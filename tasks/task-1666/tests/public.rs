use task_1666::*;

#[test]
fn public_examples() {
    assert_eq!(minor_version_descending_steps(""), 0);
    assert_eq!(minor_version_descending_steps("0.0.1"), 0);
}
