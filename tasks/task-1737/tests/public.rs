use task_1737::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_distinct(&[]), vec![]);
    assert_eq!(population_macro_distinct(&[4]), vec![1u32]);
}
