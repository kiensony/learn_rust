use task_135::*;

#[test]
fn edge_cases() {
    assert_eq!(top_up_5(0, 2), 0);
    assert_eq!(top_up_5(1, 0), 4);
    assert_eq!(top_up_5(4, 1), 2);
    assert_eq!(top_up_5(5, 2), 2);
    assert_eq!(top_up_5(6, 0), 4);
    assert_eq!(top_up_5(65535, 1), 1);
    assert_eq!(top_up_5(20, 2), 2);
    assert_eq!(top_up_5(40, 0), 0);
    assert_eq!(top_up_5(50, 1), 1);
    assert_eq!(top_up_5(100, 2), 2);
    assert_eq!(top_up_5(101, 0), 4);
    assert_eq!(top_up_5(255, 1), 1);
    assert_eq!(top_up_5(256, 2), 1);
    assert_eq!(top_up_5(0, 9), 0);
    assert_eq!(top_up_5(100, 0), 0);
    assert_eq!(top_up_5(100, 1), 1);
    assert_eq!(top_up_5(0, 0), 0);
    assert_eq!(top_up_5(5, 0), 0);
    assert_eq!(top_up_5(7, 0), 3);
    assert_eq!(top_up_5(8, 0), 2);
    assert_eq!(top_up_5(10, 0), 0);
    assert_eq!(top_up_5(11, 0), 4);
    assert_eq!(top_up_5(20, 0), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_135::TRACK_PROOF, 0x52540000 | 135);
}
