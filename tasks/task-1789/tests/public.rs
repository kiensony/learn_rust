use task_1789::*;

#[test]
fn public_examples() {
    assert_eq!(token_multiplier_longest_run(&[]), 0);
    assert_eq!(token_multiplier_longest_run(&[4]), 1);
}
