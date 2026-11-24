use task_1657::*;

#[test]
fn public_examples() {
    assert_eq!(major_version_distinct(""), vec![]);
    assert_eq!(major_version_distinct("0.0.1"), vec![0u32]);
}
