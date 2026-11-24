use task_2682::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_total(&[]), 0);
    assert_eq!(universal_slice_sum_total(&[4]), 4);
}
