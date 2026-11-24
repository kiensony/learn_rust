use task_1740::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_reverse(&[]), vec![]);
    assert_eq!(population_macro_reverse(&[4]), vec![1u32]);
}
