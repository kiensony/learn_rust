use task_2378::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_steps(&[]), vec![]);
    assert_eq!(cumulative_occupancy_steps(&[4]), vec![]);
}
