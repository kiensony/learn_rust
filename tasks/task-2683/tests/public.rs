use task_2683::*;

#[test]
fn public_examples() {
    assert_eq!(universal_slice_sum_bounds(&[]), None);
    assert_eq!(universal_slice_sum_bounds(&[4]), Some((4u32, 4u32)));
}
