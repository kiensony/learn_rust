use task_1783::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_bounds(&[]), None);
    assert_eq!(token_multiplier_bounds(&[4]), Some((16u32, 16u32)));
}
