use task_164::*;

#[test]
fn edge_cases() {
    assert_eq!(alarm_band_2(0), true);
    assert_eq!(alarm_band_2(1), true);
    assert_eq!(alarm_band_2(1), true);
    assert_eq!(alarm_band_2(2), false);
    assert_eq!(alarm_band_2(3), false);
    assert_eq!(alarm_band_2(65535), true);
    assert_eq!(alarm_band_2(4), false);
    assert_eq!(alarm_band_2(5), true);
    assert_eq!(alarm_band_2(4), false);
    assert_eq!(alarm_band_2(5), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_164::TRACK_PROOF, 0x52540000 | 164);
}
