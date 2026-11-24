use task_1627::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_distinct(&[]), vec![]);
    assert_eq!(round_trip_property_distinct(&[4]), vec![0u32]);
}
