use task_1759::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_longest_run(&[]), 0);
    assert_eq!(build_multiplier_longest_run(&[4]), 1);
}
