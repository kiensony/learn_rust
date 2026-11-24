use task_1699::*;

#[test]
fn public_examples() {
    assert_eq!(
        preview_version_frequent_readings("1.0.0 1.2.0 2.0.0", 3),
        vec![0u32]
    );
    assert_eq!(preview_version_frequent_readings("", 0), vec![]);
}
