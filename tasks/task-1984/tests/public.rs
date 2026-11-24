use task_1984::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_peaks(&[]), vec![]);
    assert_eq!(atomic_record_high_peaks(&[4]), vec![0u32]);
}
