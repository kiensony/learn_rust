use task_1972::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_total(&[]), 0);
    assert_eq!(atomic_toggle_total(&[4]), 0);
}
