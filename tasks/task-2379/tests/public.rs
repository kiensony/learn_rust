use task_2379::*;

#[test]
fn public_examples() {
    assert_eq!(cumulative_occupancy_longest_run(&[]), 0);
    assert_eq!(cumulative_occupancy_longest_run(&[4]), 1);
}
