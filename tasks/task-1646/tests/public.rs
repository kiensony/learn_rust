use task_1646::*;

#[test]
fn public_examples() {
    assert_eq!(
        parity_property_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 0))
    );
    assert_eq!(parity_property_threshold_crossing(&[], 0), None);
}
