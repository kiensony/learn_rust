use task_2689::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_longest_run(&[]), 0);
    assert_eq!(universal_slice_sum_longest_run(&[4]), 1);
}
