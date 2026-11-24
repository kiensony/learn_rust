use task_1731::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_trace(&[]), vec![]);
    assert_eq!(population_macro_trace(&[4]), vec![1u32]);
}
