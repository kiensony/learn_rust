use task_209::*;

#[test]
fn edge_cases() {
    assert_eq!(status_lamp_10(65535, 0), "low");
    assert_eq!(status_lamp_10(1, 2), "low");
    assert_eq!(status_lamp_10(65535, 9), "low");
    assert_eq!(status_lamp_10(10, 11), "ready");
    assert_eq!(status_lamp_10(65535, 11), "ready");
    assert_eq!(status_lamp_10(65535, 65535), "full");
    assert_eq!(status_lamp_10(65535, 12), "ready");
    assert_eq!(status_lamp_10(65535, 20), "full");
    assert_eq!(status_lamp_10(21, 22), "full");
    assert_eq!(status_lamp_10(1, 0), "low");
    assert_eq!(status_lamp_10(0, 1), "low");
    assert_eq!(status_lamp_10(0, 0), "low");
    assert_eq!(status_lamp_10(1, 1), "low");
    assert_eq!(status_lamp_10(9, 9), "low");
    assert_eq!(status_lamp_10(10, 10), "ready");
    assert_eq!(status_lamp_10(11, 11), "ready");
    assert_eq!(status_lamp_10(12, 12), "ready");
    assert_eq!(status_lamp_10(13, 13), "full");
    assert_eq!(status_lamp_10(20, 20), "full");
    assert_eq!(status_lamp_10(21, 21), "full");
}

#[test]
fn submission_marker() {
    assert_eq!(task_209::TRACK_PROOF, 0x52540000 | 209);
}
