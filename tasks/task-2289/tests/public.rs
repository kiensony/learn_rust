use task_2289::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_longest_run(&[]), 0);
    assert_eq!(weighted_pressure_longest_run(&[4]), 1);
}
