use task_169::*;

#[test]
fn edge_cases() {
    assert_eq!(status_lamp_2(0), "low");
    assert_eq!(status_lamp_2(1), "low");
    assert_eq!(status_lamp_2(1), "low");
    assert_eq!(status_lamp_2(2), "ready");
    assert_eq!(status_lamp_2(3), "ready");
    assert_eq!(status_lamp_2(65535), "full");
    assert_eq!(status_lamp_2(4), "ready");
    assert_eq!(status_lamp_2(5), "full");
    assert_eq!(status_lamp_2(4), "ready");
    assert_eq!(status_lamp_2(5), "full");
}

#[test]
fn submission_marker() {
    assert_eq!(task_169::TRACK_PROOF, 0x52540000 | 169);
}
