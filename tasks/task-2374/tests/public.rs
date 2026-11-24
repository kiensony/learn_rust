use task_2374::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_peaks(&[]), vec![]);
    assert_eq!(cumulative_occupancy_peaks(&[4]), vec![1u32]);
}
