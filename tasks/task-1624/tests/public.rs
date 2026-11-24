use task_1624::*;

#[test]
fn public_examples() {
    assert_eq!(round_trip_property_peaks(&[]), vec![]);
    assert_eq!(round_trip_property_peaks(&[4]), vec![0u32]);
}
