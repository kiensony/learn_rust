use task_2688::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_steps(&[]), vec![]);
    assert_eq!(universal_slice_sum_steps(&[4]), vec![]);
}
