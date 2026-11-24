use task_1985::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_changes(&[]), vec![]);
    assert_eq!(atomic_record_high_changes(&[4]), vec![0u32]);
}
