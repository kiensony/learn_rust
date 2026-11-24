use task_125::*;

#[test]
fn edge_cases() {
    assert_eq!(top_up_3(13, 3), 2);
    assert_eq!(top_up_3(1, 9), 2);
    assert_eq!(top_up_3(2, 12), 1);
    assert_eq!(top_up_3(3, 15), 0);
    assert_eq!(top_up_3(4, 1), 1);
    assert_eq!(top_up_3(65535, 4), 0);
    assert_eq!(top_up_3(20, 7), 0);
    assert_eq!(top_up_3(40, 10), 1);
    assert_eq!(top_up_3(50, 13), 0);
    assert_eq!(top_up_3(100, 16), 1);
    assert_eq!(top_up_3(101, 2), 2);
    assert_eq!(top_up_3(255, 5), 1);
    assert_eq!(top_up_3(256, 8), 0);
    assert_eq!(top_up_3(65535, 1), 0);
    assert_eq!(top_up_3(0, 0), 0);
    assert_eq!(top_up_3(1, 0), 2);
    assert_eq!(top_up_3(2, 0), 1);
    assert_eq!(top_up_3(3, 0), 0);
    assert_eq!(top_up_3(5, 0), 1);
    assert_eq!(top_up_3(6, 0), 0);
    assert_eq!(top_up_3(7, 0), 2);
    assert_eq!(top_up_3(20, 0), 1);
    assert_eq!(top_up_3(100, 0), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_125::TRACK_PROOF, 0x52540000 | 125);
}
