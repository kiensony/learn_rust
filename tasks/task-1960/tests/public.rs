use task_1960::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_reverse(&[]), vec![]);
    assert_eq!(atomic_running_total_reverse(&[4]), vec![0u32]);
}
