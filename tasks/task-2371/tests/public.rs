use task_2371::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_trace(&[]), vec![]);
    assert_eq!(cumulative_occupancy_trace(&[4]), vec![1u32]);
}
