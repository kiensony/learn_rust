use task_1742::*;

#[test]
fn public_examples() {
    assert_eq!(rotate_macro_total(&[]), 0);
    assert_eq!(rotate_macro_total(&[4]), 16);
}
