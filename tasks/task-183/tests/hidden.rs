use task_183::*;

#[test]
fn edge_cases() {
    assert_eq!(alarm_band_5(13, 1), true);
    assert_eq!(alarm_band_5(0, 2), true);
    assert_eq!(alarm_band_5(1, 0), true);
    assert_eq!(alarm_band_5(4, 1), true);
    assert_eq!(alarm_band_5(5, 2), true);
    assert_eq!(alarm_band_5(65535, 1), true);
    assert_eq!(alarm_band_5(7, 2), false);
    assert_eq!(alarm_band_5(8, 0), true);
    assert_eq!(alarm_band_5(10, 1), true);
    assert_eq!(alarm_band_5(11, 2), true);
    assert_eq!(alarm_band_5(0, 9), true);
    assert_eq!(alarm_band_5(100, 0), true);
    assert_eq!(alarm_band_5(100, 1), true);
    assert_eq!(alarm_band_5(0, 0), true);
    assert_eq!(alarm_band_5(5, 0), false);
    assert_eq!(alarm_band_5(7, 0), false);
    assert_eq!(alarm_band_5(10, 0), true);
    assert_eq!(alarm_band_5(11, 0), true);
    assert_eq!(alarm_band_5(20, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_183::TRACK_PROOF, 0x52540000 | 183);
}
