use task_1725::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_changes(&[]), vec![]);
    assert_eq!(boolean_macro_changes(&[4]), vec![0u32]);
}
