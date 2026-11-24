use task_1724::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_peaks(&[]), vec![]);
    assert_eq!(boolean_macro_peaks(&[4]), vec![0u32]);
}
