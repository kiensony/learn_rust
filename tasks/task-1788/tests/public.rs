use task_1788::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_steps(&[]), vec![]);
    assert_eq!(token_multiplier_steps(&[4]), vec![]);
}
