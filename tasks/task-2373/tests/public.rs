use task_2373::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_bounds(&[]), None);
    assert_eq!(cumulative_occupancy_bounds(&[4]), Some((1u32, 1u32)));
}
