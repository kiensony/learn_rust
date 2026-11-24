use task_1415::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_changes(&[]), vec![]);
    assert_eq!(elapsed_seconds_changes(&[4]), vec![0u32]);
}
