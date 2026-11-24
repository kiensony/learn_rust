use task_1628::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_steps(&[]), vec![]);
    assert_eq!(round_trip_property_steps(&[4]), vec![]);
}
