use task_1623::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_bounds(&[]), None);
    assert_eq!(round_trip_property_bounds(&[4]), Some((0u32, 0u32)));
}
