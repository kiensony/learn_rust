use task_1729::*;

#[test]
fn public_examples() {
    assert_eq!(boolean_macro_longest_run(&[]), 0);
    assert_eq!(boolean_macro_longest_run(&[4]), 1);
}
