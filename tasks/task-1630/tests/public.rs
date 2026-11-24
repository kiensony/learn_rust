use task_1630::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_reverse(&[]), vec![]);
    assert_eq!(round_trip_property_reverse(&[4]), vec![0u32]);
}
