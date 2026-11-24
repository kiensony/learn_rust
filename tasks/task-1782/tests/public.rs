use task_1782::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_total(&[]), 0);
    assert_eq!(token_multiplier_total(&[4]), 16);
}
