use task_1702::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_total(&[]), 0);
    assert_eq!(nibble_macro_total(&[4]), 4);
}
