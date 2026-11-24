use task_1957::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_distinct(&[]), vec![]);
    assert_eq!(atomic_running_total_distinct(&[4]), vec![0u32]);
}
