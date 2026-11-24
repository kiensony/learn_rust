use task_1981::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_record_high_trace(&[]), vec![]);
    assert_eq!(atomic_record_high_trace(&[4]), vec![0u32]);
}
