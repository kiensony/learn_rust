use task_095::*;

#[test]
fn edge_cases() {
    assert_eq!(mirror_a_coordinate_8(13, 1), 4);
    assert_eq!(mirror_a_coordinate_8(1, 3), 6);
    assert_eq!(mirror_a_coordinate_8(7, 4), 5);
    assert_eq!(mirror_a_coordinate_8(8, 5), 5);
    assert_eq!(mirror_a_coordinate_8(9, 6), 5);
    assert_eq!(mirror_a_coordinate_8(65535, 0), 65527);
    assert_eq!(mirror_a_coordinate_8(18, 3), 7);
    assert_eq!(mirror_a_coordinate_8(3, 18), 7);
    assert_eq!(mirror_a_coordinate_8(4, 4), 8);
    assert_eq!(mirror_a_coordinate_8(0, 0), 8);
    assert_eq!(mirror_a_coordinate_8(1, 0), 7);
    assert_eq!(mirror_a_coordinate_8(7, 0), 1);
    assert_eq!(mirror_a_coordinate_8(8, 0), 0);
    assert_eq!(mirror_a_coordinate_8(9, 0), 1);
    assert_eq!(mirror_a_coordinate_8(10, 0), 2);
    assert_eq!(mirror_a_coordinate_8(11, 0), 3);
    assert_eq!(mirror_a_coordinate_8(16, 0), 8);
    assert_eq!(mirror_a_coordinate_8(17, 0), 9);
    assert_eq!(mirror_a_coordinate_8(20, 0), 12);
    assert_eq!(mirror_a_coordinate_8(100, 0), 92);
}

#[test]
fn submission_marker() {
    assert_eq!(task_095::TRACK_PROOF, 0x52540000 | 95);
}
