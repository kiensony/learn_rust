use task_1721::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_trace(&[]), vec![]);
    assert_eq!(boolean_macro_trace(&[4]), vec![0u32]);
}
