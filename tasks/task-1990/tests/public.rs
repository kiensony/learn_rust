use task_1990::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_reverse(&[]), vec![]);
    assert_eq!(atomic_record_high_reverse(&[4]), vec![0u32]);
}
