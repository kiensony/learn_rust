use task_2896::*;

#[test]
fn edge_cases() {
    assert_eq!(little_endian_registers_threshold_crossing(&[4], 0), None);
    assert_eq!(
        little_endian_registers_threshold_crossing(&[9, 2, 2, 8, 0, 5], 1),
        Some((0, 521))
    );
    assert_eq!(
        little_endian_registers_threshold_crossing(&[0, 0, 0], 0),
        Some((0, 0))
    );
    assert_eq!(
        little_endian_registers_threshold_crossing(&[255, 1, 255, 2], 256),
        Some((0, 511))
    );
    assert_eq!(
        little_endian_registers_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 1539))
    );
    assert_eq!(
        little_endian_registers_threshold_crossing(&[17, 17, 1, 0, 2], 1),
        Some((0, 4369))
    );
    assert_eq!(
        little_endian_registers_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 7945),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2896::TRACK_PROOF, 0x52540000 | 2896);
}
