use task_193::*;

#[test]
fn edge_cases() {
    assert_eq!(alarm_band_8(13, 1), true);
    assert_eq!(alarm_band_8(0, 2), true);
    assert_eq!(alarm_band_8(1, 3), true);
    assert_eq!(alarm_band_8(7, 4), true);
    assert_eq!(alarm_band_8(8, 5), true);
    assert_eq!(alarm_band_8(9, 6), true);
    assert_eq!(alarm_band_8(65535, 0), true);
    assert_eq!(alarm_band_8(11, 2), false);
    assert_eq!(alarm_band_8(16, 3), true);
    assert_eq!(alarm_band_8(17, 4), true);
    assert_eq!(alarm_band_8(18, 3), true);
    assert_eq!(alarm_band_8(3, 18), true);
    assert_eq!(alarm_band_8(4, 4), true);
    assert_eq!(alarm_band_8(0, 0), true);
    assert_eq!(alarm_band_8(1, 0), true);
    assert_eq!(alarm_band_8(7, 0), true);
    assert_eq!(alarm_band_8(8, 0), false);
    assert_eq!(alarm_band_8(9, 0), false);
    assert_eq!(alarm_band_8(10, 0), false);
    assert_eq!(alarm_band_8(11, 0), true);
    assert_eq!(alarm_band_8(16, 0), true);
    assert_eq!(alarm_band_8(17, 0), true);
    assert_eq!(alarm_band_8(20, 0), true);
    assert_eq!(alarm_band_8(100, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_193::TRACK_PROOF, 0x52540000 | 193);
}
