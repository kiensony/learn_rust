use task_2372::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_total(&[]), 0);
    assert_eq!(cumulative_occupancy_total(&[4]), 1);
}
