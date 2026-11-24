use task_1701::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_trace(&[]), vec![]);
    assert_eq!(nibble_macro_trace(&[4]), vec![4u32]);
}
