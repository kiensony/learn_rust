use task_1781::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_trace(&[]), vec![]);
    assert_eq!(token_multiplier_trace(&[4]), vec![16u32]);
}
