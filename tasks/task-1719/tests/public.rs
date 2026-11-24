use task_1719::*;

#[test]
fn public_examples() {
    assert_eq!(saturating_macro_longest_run(&[]), 0);
    assert_eq!(saturating_macro_longest_run(&[4]), 1);
}
