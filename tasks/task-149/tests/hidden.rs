use task_149::*;

#[test]
fn edge_cases() {
    assert_eq!(nearest_group_8(0, 2), 0);
    assert_eq!(nearest_group_8(1, 3), 0);
    assert_eq!(nearest_group_8(7, 4), 0);
    assert_eq!(nearest_group_8(8, 5), 0);
    assert_eq!(nearest_group_8(9, 6), 0);
    assert_eq!(nearest_group_8(65535, 0), 65536);
    assert_eq!(nearest_group_8(20, 1), 16);
    assert_eq!(nearest_group_8(40, 2), 40);
    assert_eq!(nearest_group_8(50, 3), 48);
    assert_eq!(nearest_group_8(100, 4), 96);
    assert_eq!(nearest_group_8(101, 5), 96);
    assert_eq!(nearest_group_8(255, 6), 248);
    assert_eq!(nearest_group_8(256, 0), 256);
    assert_eq!(nearest_group_8(18, 3), 16);
    assert_eq!(nearest_group_8(3, 18), 16);
    assert_eq!(nearest_group_8(4, 4), 0);
    assert_eq!(nearest_group_8(0, 0), 0);
    assert_eq!(nearest_group_8(1, 0), 0);
    assert_eq!(nearest_group_8(7, 0), 8);
    assert_eq!(nearest_group_8(8, 0), 8);
    assert_eq!(nearest_group_8(9, 0), 8);
    assert_eq!(nearest_group_8(10, 0), 8);
    assert_eq!(nearest_group_8(11, 0), 8);
    assert_eq!(nearest_group_8(16, 0), 16);
    assert_eq!(nearest_group_8(17, 0), 16);
    assert_eq!(nearest_group_8(20, 0), 24);
    assert_eq!(nearest_group_8(100, 0), 104);
}

#[test]
fn submission_marker() {
    assert_eq!(task_149::TRACK_PROOF, 0x52540000 | 149);
}
