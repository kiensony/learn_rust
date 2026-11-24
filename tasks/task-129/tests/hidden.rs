use task_129::*;

#[test]
fn edge_cases() {
    assert_eq!(nearest_group_3(0, 6), 6);
    assert_eq!(nearest_group_3(1, 9), 9);
    assert_eq!(nearest_group_3(2, 12), 15);
    assert_eq!(nearest_group_3(3, 15), 18);
    assert_eq!(nearest_group_3(4, 1), 6);
    assert_eq!(nearest_group_3(65535, 4), 65535);
    assert_eq!(nearest_group_3(20, 7), 27);
    assert_eq!(nearest_group_3(40, 10), 51);
    assert_eq!(nearest_group_3(50, 13), 63);
    assert_eq!(nearest_group_3(100, 16), 117);
    assert_eq!(nearest_group_3(101, 2), 102);
    assert_eq!(nearest_group_3(255, 5), 261);
    assert_eq!(nearest_group_3(256, 8), 264);
    assert_eq!(nearest_group_3(65535, 1), 65535);
    assert_eq!(nearest_group_3(0, 0), 0);
    assert_eq!(nearest_group_3(1, 0), 0);
    assert_eq!(nearest_group_3(2, 0), 3);
    assert_eq!(nearest_group_3(3, 0), 3);
    assert_eq!(nearest_group_3(5, 0), 6);
    assert_eq!(nearest_group_3(6, 0), 6);
    assert_eq!(nearest_group_3(7, 0), 6);
    assert_eq!(nearest_group_3(20, 0), 21);
    assert_eq!(nearest_group_3(100, 0), 99);
}

#[test]
fn submission_marker() {
    assert_eq!(task_129::TRACK_PROOF, 0x52540000 | 129);
}
