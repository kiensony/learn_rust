use task_173::*;

#[test]
fn edge_cases() {
    assert_eq!(alarm_band_3(0, 6), true);
    assert_eq!(alarm_band_3(1, 9), true);
    assert_eq!(alarm_band_3(2, 12), true);
    assert_eq!(alarm_band_3(3, 15), true);
    assert_eq!(alarm_band_3(4, 1), false);
    assert_eq!(alarm_band_3(65535, 4), true);
    assert_eq!(alarm_band_3(5, 7), true);
    assert_eq!(alarm_band_3(6, 10), true);
    assert_eq!(alarm_band_3(6, 13), true);
    assert_eq!(alarm_band_3(7, 16), true);
    assert_eq!(alarm_band_3(65535, 1), true);
    assert_eq!(alarm_band_3(0, 0), true);
    assert_eq!(alarm_band_3(1, 0), true);
    assert_eq!(alarm_band_3(2, 0), true);
    assert_eq!(alarm_band_3(3, 0), false);
    assert_eq!(alarm_band_3(5, 0), false);
    assert_eq!(alarm_band_3(6, 0), true);
    assert_eq!(alarm_band_3(7, 0), true);
    assert_eq!(alarm_band_3(20, 0), true);
    assert_eq!(alarm_band_3(100, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_173::TRACK_PROOF, 0x52540000 | 173);
}
