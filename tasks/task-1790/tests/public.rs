use task_1790::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_reverse(&[]), vec![]);
    assert_eq!(token_multiplier_reverse(&[4]), vec![16u32]);
}
