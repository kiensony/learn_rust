use task_1730::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_reverse(&[]), vec![]);
    assert_eq!(boolean_macro_reverse(&[4]), vec![0u32]);
}
