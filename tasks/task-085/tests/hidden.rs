use task_085::*;

#[test]
fn edge_cases() {
    assert_eq!(mirror_a_coordinate_5(0, 2), 5);
    assert_eq!(mirror_a_coordinate_5(1, 0), 4);
    assert_eq!(mirror_a_coordinate_5(4, 1), 2);
    assert_eq!(mirror_a_coordinate_5(5, 2), 2);
    assert_eq!(mirror_a_coordinate_5(6, 0), 1);
    assert_eq!(mirror_a_coordinate_5(65535, 1), 65529);
    assert_eq!(mirror_a_coordinate_5(0, 9), 5);
    assert_eq!(mirror_a_coordinate_5(100, 0), 95);
    assert_eq!(mirror_a_coordinate_5(100, 1), 94);
    assert_eq!(mirror_a_coordinate_5(0, 0), 5);
    assert_eq!(mirror_a_coordinate_5(5, 0), 0);
    assert_eq!(mirror_a_coordinate_5(7, 0), 2);
    assert_eq!(mirror_a_coordinate_5(8, 0), 3);
    assert_eq!(mirror_a_coordinate_5(10, 0), 5);
    assert_eq!(mirror_a_coordinate_5(11, 0), 6);
    assert_eq!(mirror_a_coordinate_5(20, 0), 15);
}

#[test]
fn submission_marker() {
    assert_eq!(task_085::TRACK_PROOF, 0x52540000 | 85);
}
