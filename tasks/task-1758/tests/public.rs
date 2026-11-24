use task_1758::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_steps(&[]), vec![]);
    assert_eq!(build_multiplier_steps(&[4]), vec![]);
}
