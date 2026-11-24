use task_2375::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_changes(&[]), vec![]);
    assert_eq!(cumulative_occupancy_changes(&[4]), vec![1u32]);
}
