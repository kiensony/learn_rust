use task_189::*;

#[test]
fn edge_cases() {
    assert_eq!(status_lamp_5(0, 2), "low");
    assert_eq!(status_lamp_5(1, 0), "low");
    assert_eq!(status_lamp_5(4, 1), "low");
    assert_eq!(status_lamp_5(5, 2), "low");
    assert_eq!(status_lamp_5(6, 0), "ready");
    assert_eq!(status_lamp_5(65535, 1), "full");
    assert_eq!(status_lamp_5(7, 2), "ready");
    assert_eq!(status_lamp_5(8, 0), "full");
    assert_eq!(status_lamp_5(10, 1), "full");
    assert_eq!(status_lamp_5(11, 2), "full");
    assert_eq!(status_lamp_5(0, 9), "low");
    assert_eq!(status_lamp_5(100, 0), "full");
    assert_eq!(status_lamp_5(100, 1), "full");
    assert_eq!(status_lamp_5(0, 0), "low");
    assert_eq!(status_lamp_5(5, 0), "ready");
    assert_eq!(status_lamp_5(7, 0), "ready");
    assert_eq!(status_lamp_5(10, 0), "full");
    assert_eq!(status_lamp_5(11, 0), "full");
    assert_eq!(status_lamp_5(20, 0), "full");
}

#[test]
fn submission_marker() {
    assert_eq!(task_189::TRACK_PROOF, 0x52540000 | 189);
}
