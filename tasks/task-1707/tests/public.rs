use task_1707::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_distinct(&[]), vec![]);
    assert_eq!(nibble_macro_distinct(&[4]), vec![4u32]);
}
