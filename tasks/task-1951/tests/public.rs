use task_1951::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_trace(&[]), vec![]);
    assert_eq!(atomic_running_total_trace(&[4]), vec![0u32]);
}
