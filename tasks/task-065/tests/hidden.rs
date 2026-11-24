use task_065::*;

#[test]
fn edge_cases() {
    assert_eq!(mirror_a_coordinate_2(0), 2);
    assert_eq!(mirror_a_coordinate_2(1), 1);
    assert_eq!(mirror_a_coordinate_2(1), 1);
    assert_eq!(mirror_a_coordinate_2(2), 0);
    assert_eq!(mirror_a_coordinate_2(3), 1);
    assert_eq!(mirror_a_coordinate_2(65535), 65533);
}

#[test]
fn submission_marker() {
    assert_eq!(task_065::TRACK_PROOF, 0x52540000 | 65);
}
