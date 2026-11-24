use task_1787::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_distinct(&[]), vec![]);
    assert_eq!(token_multiplier_distinct(&[4]), vec![16u32]);
}
