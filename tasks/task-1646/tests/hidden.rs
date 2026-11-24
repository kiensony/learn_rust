use task_1646::*;

#[test]
fn edge_cases() {
    assert_eq!(parity_property_threshold_crossing(&[4], 0), Some((0, 0)));
    assert_eq!(
        parity_property_threshold_crossing(&[9, 2, 2, 8, 0, 5], 1),
        None
    );
    assert_eq!(
        parity_property_threshold_crossing(&[0, 0, 0], 0),
        Some((0, 0))
    );
    assert_eq!(
        parity_property_threshold_crossing(&[255, 1, 255, 2], 256),
        None
    );
    assert_eq!(
        parity_property_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 0))
    );
    assert_eq!(
        parity_property_threshold_crossing(&[17, 17, 1, 0, 2], 1),
        None
    );
    assert_eq!(
        parity_property_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1646::TRACK_PROOF, 0x52540000 | 1646);
}
