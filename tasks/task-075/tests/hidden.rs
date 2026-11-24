use task_075::*;

#[test]
fn edge_cases() {
    assert_eq!(mirror_a_coordinate_3(0, 6), 3);
    assert_eq!(mirror_a_coordinate_3(1, 9), 7);
    assert_eq!(mirror_a_coordinate_3(2, 12), 11);
    assert_eq!(mirror_a_coordinate_3(3, 15), 15);
    assert_eq!(mirror_a_coordinate_3(4, 1), 2);
    assert_eq!(mirror_a_coordinate_3(65535, 4), 65532);
    assert_eq!(mirror_a_coordinate_3(65535, 1), 65532);
    assert_eq!(mirror_a_coordinate_3(0, 0), 3);
    assert_eq!(mirror_a_coordinate_3(1, 0), 2);
    assert_eq!(mirror_a_coordinate_3(2, 0), 1);
    assert_eq!(mirror_a_coordinate_3(3, 0), 0);
    assert_eq!(mirror_a_coordinate_3(5, 0), 2);
    assert_eq!(mirror_a_coordinate_3(6, 0), 3);
    assert_eq!(mirror_a_coordinate_3(7, 0), 4);
    assert_eq!(mirror_a_coordinate_3(20, 0), 17);
    assert_eq!(mirror_a_coordinate_3(100, 0), 97);
}

#[test]
fn submission_marker() {
    assert_eq!(task_075::TRACK_PROOF, 0x52540000 | 75);
}
