use task_145::*;

#[test]
fn edge_cases() {
    assert_eq!(top_up_8(13, 1), 4);
    assert_eq!(top_up_8(1, 3), 6);
    assert_eq!(top_up_8(7, 4), 5);
    assert_eq!(top_up_8(8, 5), 5);
    assert_eq!(top_up_8(9, 6), 5);
    assert_eq!(top_up_8(65535, 0), 1);
    assert_eq!(top_up_8(20, 1), 5);
    assert_eq!(top_up_8(40, 2), 2);
    assert_eq!(top_up_8(50, 3), 1);
    assert_eq!(top_up_8(100, 4), 0);
    assert_eq!(top_up_8(101, 5), 0);
    assert_eq!(top_up_8(255, 6), 7);
    assert_eq!(top_up_8(256, 0), 0);
    assert_eq!(top_up_8(18, 3), 1);
    assert_eq!(top_up_8(3, 18), 1);
    assert_eq!(top_up_8(4, 4), 0);
    assert_eq!(top_up_8(0, 0), 0);
    assert_eq!(top_up_8(1, 0), 7);
    assert_eq!(top_up_8(7, 0), 1);
    assert_eq!(top_up_8(8, 0), 0);
    assert_eq!(top_up_8(9, 0), 7);
    assert_eq!(top_up_8(10, 0), 6);
    assert_eq!(top_up_8(11, 0), 5);
    assert_eq!(top_up_8(16, 0), 0);
    assert_eq!(top_up_8(17, 0), 7);
    assert_eq!(top_up_8(20, 0), 4);
    assert_eq!(top_up_8(100, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_145::TRACK_PROOF, 0x52540000 | 145);
}
