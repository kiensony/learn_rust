use task_099::*;

#[test]
fn edge_cases() {
    assert_eq!(a_full_tray_8(13, 1), false);
    assert_eq!(a_full_tray_8(0, 2), false);
    assert_eq!(a_full_tray_8(1, 3), false);
    assert_eq!(a_full_tray_8(7, 4), false);
    assert_eq!(a_full_tray_8(8, 5), false);
    assert_eq!(a_full_tray_8(9, 6), false);
    assert_eq!(a_full_tray_8(65535, 0), false);
    assert_eq!(a_full_tray_8(18, 3), false);
    assert_eq!(a_full_tray_8(3, 18), false);
    assert_eq!(a_full_tray_8(0, 0), true);
    assert_eq!(a_full_tray_8(1, 0), false);
    assert_eq!(a_full_tray_8(7, 0), false);
    assert_eq!(a_full_tray_8(8, 0), true);
    assert_eq!(a_full_tray_8(9, 0), false);
    assert_eq!(a_full_tray_8(10, 0), false);
    assert_eq!(a_full_tray_8(11, 0), false);
    assert_eq!(a_full_tray_8(16, 0), true);
    assert_eq!(a_full_tray_8(17, 0), false);
    assert_eq!(a_full_tray_8(20, 0), false);
    assert_eq!(a_full_tray_8(100, 0), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_099::TRACK_PROOF, 0x52540000 | 99);
}
