use task_089::*;

#[test]
fn edge_cases() {
    assert_eq!(a_full_tray_5(13, 1), false);
    assert_eq!(a_full_tray_5(1, 0), false);
    assert_eq!(a_full_tray_5(4, 1), false);
    assert_eq!(a_full_tray_5(5, 2), false);
    assert_eq!(a_full_tray_5(6, 0), false);
    assert_eq!(a_full_tray_5(65535, 1), false);
    assert_eq!(a_full_tray_5(0, 9), true);
    assert_eq!(a_full_tray_5(100, 0), true);
    assert_eq!(a_full_tray_5(100, 1), false);
    assert_eq!(a_full_tray_5(0, 0), true);
    assert_eq!(a_full_tray_5(5, 0), true);
    assert_eq!(a_full_tray_5(7, 0), false);
    assert_eq!(a_full_tray_5(8, 0), false);
    assert_eq!(a_full_tray_5(10, 0), true);
    assert_eq!(a_full_tray_5(11, 0), false);
    assert_eq!(a_full_tray_5(20, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_089::TRACK_PROOF, 0x52540000 | 89);
}
