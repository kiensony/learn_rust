use task_1785::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_changes(&[]), vec![]);
    assert_eq!(token_multiplier_changes(&[4]), vec![16u32]);
}
