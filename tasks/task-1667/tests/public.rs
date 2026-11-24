use task_1667::*;

#[test]
fn public_examples() {
    assert_eq!(minor_version_distinct(""), vec![]);
    assert_eq!(minor_version_distinct("0.0.1"), vec![0u32]);
}
