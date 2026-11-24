use task_1412::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_total(&[]), 0);
    assert_eq!(elapsed_seconds_total(&[4]), 0);
}
