use task_1746::*;

#[test]
fn edge_cases() {
    assert_eq!(nibble_macro_threshold_crossing(&[4], 0), Some((0, 4)));
    assert_eq!(
        nibble_macro_threshold_crossing(&[9, 2, 2, 8, 0, 5], 1),
        Some((0, 9))
    );
    assert_eq!(nibble_macro_threshold_crossing(&[0, 0, 0], 0), Some((0, 0)));
    assert_eq!(
        nibble_macro_threshold_crossing(&[255, 1, 255, 2], 256),
        None
    );
    assert_eq!(
        nibble_macro_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 3))
    );
    assert_eq!(
        nibble_macro_threshold_crossing(&[17, 17, 1, 0, 2], 1),
        Some((0, 1))
    );
    assert_eq!(
        nibble_macro_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 16),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1746::TRACK_PROOF, 0x52540000 | 1746);
}
