use task_1735::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_changes(&[]), vec![]);
    assert_eq!(population_macro_changes(&[4]), vec![1u32]);
}
