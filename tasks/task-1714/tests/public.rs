use task_1714::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_peaks(&[]), vec![]);
    assert_eq!(saturating_macro_peaks(&[4]), vec![8u32]);
}
