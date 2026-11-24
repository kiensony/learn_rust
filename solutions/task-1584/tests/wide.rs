#![cfg(feature = "wide")]

#[test]
fn feature_units_are_used_by_the_public_report() {
    assert_eq!(
        task_1584::feature_lanes_peaks(&[5, 2, 2, 8, 9, 1, 14, 1, 1]),
        vec![80u32, 128u32, 144u32, 224u32]
    );
}
