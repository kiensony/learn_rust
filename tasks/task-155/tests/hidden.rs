use task_155::*;

#[test]
fn edge_cases() {
    assert_eq!(top_up_10(65535, 0), 0);
    assert_eq!(top_up_10(1, 2), 9);
    assert_eq!(top_up_10(65535, 9), 1);
    assert_eq!(top_up_10(10, 11), 0);
    assert_eq!(top_up_10(65535, 11), 9);
    assert_eq!(top_up_10(65535, 65535), 5);
    assert_eq!(top_up_10(65535, 20), 0);
    assert_eq!(top_up_10(40, 41), 0);
    assert_eq!(top_up_10(65535, 50), 0);
    assert_eq!(top_up_10(100, 101), 0);
    assert_eq!(top_up_10(65535, 101), 9);
    assert_eq!(top_up_10(255, 256), 5);
    assert_eq!(top_up_10(65535, 256), 4);
    assert_eq!(top_up_10(1, 0), 0);
    assert_eq!(top_up_10(0, 1), 0);
    assert_eq!(top_up_10(0, 0), 0);
    assert_eq!(top_up_10(1, 1), 9);
    assert_eq!(top_up_10(9, 9), 1);
    assert_eq!(top_up_10(10, 10), 0);
    assert_eq!(top_up_10(11, 11), 9);
    assert_eq!(top_up_10(12, 12), 8);
    assert_eq!(top_up_10(13, 13), 7);
    assert_eq!(top_up_10(20, 20), 0);
    assert_eq!(top_up_10(21, 21), 9);
    assert_eq!(top_up_10(100, 100), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_155::TRACK_PROOF, 0x52540000 | 155);
}
