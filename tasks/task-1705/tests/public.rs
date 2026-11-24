use task_1705::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_changes(&[]), vec![]);
    assert_eq!(nibble_macro_changes(&[4]), vec![4u32]);
}
