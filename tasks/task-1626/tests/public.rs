use task_1626::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_descending_steps(&[]), 0);
    assert_eq!(round_trip_property_descending_steps(&[4]), 0);
}
