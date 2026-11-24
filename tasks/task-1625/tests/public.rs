use task_1625::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_changes(&[]), vec![]);
    assert_eq!(round_trip_property_changes(&[4]), vec![0u32]);
}
