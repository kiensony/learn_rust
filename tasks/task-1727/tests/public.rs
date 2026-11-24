use task_1727::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_distinct(&[]), vec![]);
    assert_eq!(boolean_macro_distinct(&[4]), vec![0u32]);
}
