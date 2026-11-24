use task_1622::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_total(&[]), 0);
    assert_eq!(round_trip_property_total(&[4]), 0);
}
