use task_1987::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_distinct(&[]), vec![]);
    assert_eq!(atomic_record_high_distinct(&[4]), vec![0u32]);
}
