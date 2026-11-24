use task_1745::*;

#[test]
fn public_examples() {
    assert_eq!(rotate_macro_changes(&[]), vec![]);
    assert_eq!(rotate_macro_changes(&[4]), vec![16u32]);
}
