use task_1710::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_reverse(&[]), vec![]);
    assert_eq!(nibble_macro_reverse(&[4]), vec![4u32]);
}
