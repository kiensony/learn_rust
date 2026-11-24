use task_1866::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_descending_steps(&[]), 0);
    assert_eq!(weak_read_handles_descending_steps(&[4]), 0);
}
