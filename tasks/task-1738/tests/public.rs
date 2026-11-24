use task_1738::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_steps(&[]), vec![]);
    assert_eq!(population_macro_steps(&[4]), vec![]);
}
