use task_1956::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_running_total_descending_steps(&[]), 0);
    assert_eq!(atomic_running_total_descending_steps(&[4]), 0);
}
