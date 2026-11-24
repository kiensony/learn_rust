use task_1708::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_macro_steps(&[]), vec![]);
    assert_eq!(nibble_macro_steps(&[4]), vec![]);
}
