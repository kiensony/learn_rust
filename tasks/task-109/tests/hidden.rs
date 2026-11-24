use task_109::*;

#[test]
fn edge_cases() {
    assert_eq!(a_full_tray_10(13, 14), false);
    assert_eq!(a_full_tray_10(1, 2), false);
    assert_eq!(a_full_tray_10(65535, 9), false);
    assert_eq!(a_full_tray_10(10, 11), true);
    assert_eq!(a_full_tray_10(65535, 11), false);
    assert_eq!(a_full_tray_10(65535, 65535), false);
    assert_eq!(a_full_tray_10(1, 0), true);
    assert_eq!(a_full_tray_10(0, 1), true);
    assert_eq!(a_full_tray_10(0, 0), true);
    assert_eq!(a_full_tray_10(1, 1), false);
    assert_eq!(a_full_tray_10(9, 9), false);
    assert_eq!(a_full_tray_10(10, 10), true);
    assert_eq!(a_full_tray_10(11, 11), false);
    assert_eq!(a_full_tray_10(12, 12), false);
    assert_eq!(a_full_tray_10(13, 13), false);
    assert_eq!(a_full_tray_10(20, 20), true);
    assert_eq!(a_full_tray_10(21, 21), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_109::TRACK_PROOF, 0x52540000 | 109);
}
