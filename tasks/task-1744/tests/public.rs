use task_1744::*;

#[test]
fn public_examples() {
    assert_eq!(rotate_macro_peaks(&[]), vec![]);
    assert_eq!(rotate_macro_peaks(&[4]), vec![16u32]);
}
