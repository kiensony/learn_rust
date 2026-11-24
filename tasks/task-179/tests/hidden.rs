use task_179::*;

#[test]
fn edge_cases() {
    assert_eq!(status_lamp_3(0, 6), "full");
    assert_eq!(status_lamp_3(1, 9), "full");
    assert_eq!(status_lamp_3(2, 12), "full");
    assert_eq!(status_lamp_3(3, 15), "full");
    assert_eq!(status_lamp_3(4, 1), "ready");
    assert_eq!(status_lamp_3(65535, 4), "full");
    assert_eq!(status_lamp_3(5, 7), "full");
    assert_eq!(status_lamp_3(6, 10), "full");
    assert_eq!(status_lamp_3(6, 13), "full");
    assert_eq!(status_lamp_3(7, 16), "full");
    assert_eq!(status_lamp_3(65535, 1), "full");
    assert_eq!(status_lamp_3(0, 0), "low");
    assert_eq!(status_lamp_3(1, 0), "low");
    assert_eq!(status_lamp_3(2, 0), "low");
    assert_eq!(status_lamp_3(3, 0), "ready");
    assert_eq!(status_lamp_3(5, 0), "ready");
    assert_eq!(status_lamp_3(6, 0), "full");
    assert_eq!(status_lamp_3(7, 0), "full");
    assert_eq!(status_lamp_3(20, 0), "full");
    assert_eq!(status_lamp_3(100, 0), "full");
}

#[test]
fn submission_marker() {
    assert_eq!(task_179::TRACK_PROOF, 0x52540000 | 179);
}
