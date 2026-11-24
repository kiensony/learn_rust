use task_1952::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_total(&[]), 0);
    assert_eq!(atomic_running_total_total(&[4]), 0);
}
