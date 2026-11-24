use task_1732::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_total(&[]), 0);
    assert_eq!(population_macro_total(&[4]), 1);
}
