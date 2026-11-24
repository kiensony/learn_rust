use task_105::*;

#[test]
fn edge_cases() {
    assert_eq!(mirror_a_coordinate_10(65535, 0), 10);
    assert_eq!(mirror_a_coordinate_10(1, 2), 9);
    assert_eq!(mirror_a_coordinate_10(65535, 9), 1);
    assert_eq!(mirror_a_coordinate_10(10, 11), 0);
    assert_eq!(mirror_a_coordinate_10(65535, 11), 1);
    assert_eq!(mirror_a_coordinate_10(65535, 65535), 65525);
    assert_eq!(mirror_a_coordinate_10(1, 0), 10);
    assert_eq!(mirror_a_coordinate_10(0, 1), 10);
    assert_eq!(mirror_a_coordinate_10(0, 0), 10);
    assert_eq!(mirror_a_coordinate_10(1, 1), 9);
    assert_eq!(mirror_a_coordinate_10(9, 9), 1);
    assert_eq!(mirror_a_coordinate_10(10, 10), 0);
    assert_eq!(mirror_a_coordinate_10(11, 11), 1);
    assert_eq!(mirror_a_coordinate_10(12, 12), 2);
    assert_eq!(mirror_a_coordinate_10(13, 13), 3);
    assert_eq!(mirror_a_coordinate_10(20, 20), 10);
    assert_eq!(mirror_a_coordinate_10(21, 21), 11);
    assert_eq!(mirror_a_coordinate_10(100, 100), 90);
}

#[test]
fn submission_marker() {
    assert_eq!(task_105::TRACK_PROOF, 0x52540000 | 105);
}
