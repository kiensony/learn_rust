use task_1856::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_descending_steps(&[]), 0);
    assert_eq!(shared_read_handles_descending_steps(&[4]), 0);
}
