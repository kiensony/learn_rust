use task_139::*;

#[test]
fn edge_cases() {
    assert_eq!(nearest_group_5(0, 2), 0);
    assert_eq!(nearest_group_5(1, 0), 0);
    assert_eq!(nearest_group_5(4, 1), 5);
    assert_eq!(nearest_group_5(5, 2), 5);
    assert_eq!(nearest_group_5(6, 0), 5);
    assert_eq!(nearest_group_5(65535, 1), 65535);
    assert_eq!(nearest_group_5(20, 2), 20);
    assert_eq!(nearest_group_5(40, 0), 40);
    assert_eq!(nearest_group_5(50, 1), 50);
    assert_eq!(nearest_group_5(100, 2), 100);
    assert_eq!(nearest_group_5(101, 0), 100);
    assert_eq!(nearest_group_5(255, 1), 255);
    assert_eq!(nearest_group_5(256, 2), 255);
    assert_eq!(nearest_group_5(0, 9), 0);
    assert_eq!(nearest_group_5(100, 0), 100);
    assert_eq!(nearest_group_5(100, 1), 100);
    assert_eq!(nearest_group_5(0, 0), 0);
    assert_eq!(nearest_group_5(5, 0), 5);
    assert_eq!(nearest_group_5(7, 0), 5);
    assert_eq!(nearest_group_5(8, 0), 10);
    assert_eq!(nearest_group_5(10, 0), 10);
    assert_eq!(nearest_group_5(11, 0), 10);
    assert_eq!(nearest_group_5(20, 0), 20);
}

#[test]
fn submission_marker() {
    assert_eq!(task_139::TRACK_PROOF, 0x52540000 | 139);
}
