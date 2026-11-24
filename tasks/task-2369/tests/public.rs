use task_2369::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_minimum_longest_run(&[]), 0);
    assert_eq!(single_pass_prefix_minimum_longest_run(&[4]), 1);
}
