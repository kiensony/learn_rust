use task_1416::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_descending_steps(&[]), 0);
    assert_eq!(elapsed_seconds_descending_steps(&[4]), 0);
}
