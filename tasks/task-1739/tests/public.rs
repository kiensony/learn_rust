use task_1739::*;

#[test]
fn public_examples() {
    assert_eq!(population_macro_longest_run(&[]), 0);
    assert_eq!(population_macro_longest_run(&[4]), 1);
}
