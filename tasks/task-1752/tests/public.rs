use task_1752::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_total(&[]), 0);
    assert_eq!(build_multiplier_total(&[4]), 12);
}
