use task_2380::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_reverse(&[]), vec![]);
    assert_eq!(cumulative_occupancy_reverse(&[4]), vec![1u32]);
}
