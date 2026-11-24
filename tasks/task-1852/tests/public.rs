use task_1852::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_total(&[]), 0);
    assert_eq!(shared_read_handles_total(&[4]), 1);
}
