use task_115::*;

#[test]
fn edge_cases() {
    assert_eq!(top_up_2(0), 0);
    assert_eq!(top_up_2(1), 1);
    assert_eq!(top_up_2(1), 1);
    assert_eq!(top_up_2(2), 0);
    assert_eq!(top_up_2(3), 1);
    assert_eq!(top_up_2(65535), 1);
    assert_eq!(top_up_2(20), 0);
    assert_eq!(top_up_2(40), 0);
    assert_eq!(top_up_2(50), 0);
    assert_eq!(top_up_2(100), 0);
    assert_eq!(top_up_2(101), 1);
    assert_eq!(top_up_2(255), 1);
    assert_eq!(top_up_2(256), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_115::TRACK_PROOF, 0x52540000 | 115);
}
