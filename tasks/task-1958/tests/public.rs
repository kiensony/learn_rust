use task_1958::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_steps(&[]), vec![]);
    assert_eq!(atomic_running_total_steps(&[4]), vec![]);
}
