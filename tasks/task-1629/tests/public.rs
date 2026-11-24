use task_1629::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_longest_run(&[]), 0);
    assert_eq!(round_trip_property_longest_run(&[4]), 1);
}
