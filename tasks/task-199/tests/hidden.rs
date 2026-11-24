use task_199::*;

#[test]
fn edge_cases() {
    assert_eq!(status_lamp_8(0, 2), "low");
    assert_eq!(status_lamp_8(1, 3), "low");
    assert_eq!(status_lamp_8(7, 4), "low");
    assert_eq!(status_lamp_8(8, 5), "low");
    assert_eq!(status_lamp_8(9, 6), "low");
    assert_eq!(status_lamp_8(65535, 0), "full");
    assert_eq!(status_lamp_8(10, 1), "ready");
    assert_eq!(status_lamp_8(11, 2), "ready");
    assert_eq!(status_lamp_8(16, 3), "full");
    assert_eq!(status_lamp_8(17, 4), "full");
    assert_eq!(status_lamp_8(18, 3), "full");
    assert_eq!(status_lamp_8(3, 18), "full");
    assert_eq!(status_lamp_8(4, 4), "low");
    assert_eq!(status_lamp_8(0, 0), "low");
    assert_eq!(status_lamp_8(1, 0), "low");
    assert_eq!(status_lamp_8(7, 0), "low");
    assert_eq!(status_lamp_8(8, 0), "ready");
    assert_eq!(status_lamp_8(9, 0), "ready");
    assert_eq!(status_lamp_8(10, 0), "ready");
    assert_eq!(status_lamp_8(11, 0), "full");
    assert_eq!(status_lamp_8(16, 0), "full");
    assert_eq!(status_lamp_8(17, 0), "full");
    assert_eq!(status_lamp_8(20, 0), "full");
    assert_eq!(status_lamp_8(100, 0), "full");
}

#[test]
fn submission_marker() {
    assert_eq!(task_199::TRACK_PROOF, 0x52540000 | 199);
}
