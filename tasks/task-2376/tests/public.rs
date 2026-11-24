use task_2376::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_descending_steps(&[]), 0);
    assert_eq!(cumulative_occupancy_descending_steps(&[4]), 0);
}
