use task_2359::*;

#[test]
fn public_examples() {
    assert_eq!(single_pass_prefix_maximum_longest_run(&[]), 0);
    assert_eq!(single_pass_prefix_maximum_longest_run(&[4]), 1);
}
