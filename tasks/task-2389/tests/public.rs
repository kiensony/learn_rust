use task_2389::*;

#[test]
fn public_examples() {
    assert_eq!(rolling_parity_longest_run(&[]), 0);
    assert_eq!(rolling_parity_longest_run(&[4]), 1);
}
