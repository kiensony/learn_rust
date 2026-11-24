use task_203::*;

#[test]
fn edge_cases() {
    assert_eq!(alarm_band_10(13, 14), true);
    assert_eq!(alarm_band_10(65535, 0), true);
    assert_eq!(alarm_band_10(1, 2), true);
    assert_eq!(alarm_band_10(65535, 9), true);
    assert_eq!(alarm_band_10(65535, 11), false);
    assert_eq!(alarm_band_10(65535, 65535), true);
    assert_eq!(alarm_band_10(65535, 12), false);
    assert_eq!(alarm_band_10(65535, 20), true);
    assert_eq!(alarm_band_10(21, 22), true);
    assert_eq!(alarm_band_10(1, 0), true);
    assert_eq!(alarm_band_10(0, 1), true);
    assert_eq!(alarm_band_10(0, 0), true);
    assert_eq!(alarm_band_10(1, 1), true);
    assert_eq!(alarm_band_10(9, 9), true);
    assert_eq!(alarm_band_10(10, 10), false);
    assert_eq!(alarm_band_10(11, 11), false);
    assert_eq!(alarm_band_10(12, 12), false);
    assert_eq!(alarm_band_10(13, 13), true);
    assert_eq!(alarm_band_10(20, 20), true);
    assert_eq!(alarm_band_10(21, 21), true);
    assert_eq!(alarm_band_10(100, 100), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_203::TRACK_PROOF, 0x52540000 | 203);
}
