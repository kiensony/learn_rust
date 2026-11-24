use task_1704::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_peaks(&[]), vec![]);
    assert_eq!(nibble_macro_peaks(&[4]), vec![4u32]);
}
