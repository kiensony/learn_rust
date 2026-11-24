use task_1862::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_total(&[]), 0);
    assert_eq!(weak_read_handles_total(&[4]), 0);
}
