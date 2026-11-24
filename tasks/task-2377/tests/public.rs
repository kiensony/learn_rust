use task_2377::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_distinct(&[]), vec![]);
    assert_eq!(cumulative_occupancy_distinct(&[4]), vec![1u32]);
}
