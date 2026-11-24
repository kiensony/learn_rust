use task_1784::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_peaks(&[]), vec![]);
    assert_eq!(token_multiplier_peaks(&[4]), vec![16u32]);
}
