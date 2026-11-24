use task_1939::*;

#[test]
fn public_examples() {
    assert_eq!(locked_batch_totals_longest_run(&[]), 0);
    assert_eq!(locked_batch_totals_longest_run(&[4]), 1);
}
