use task_159::*;

#[test]
fn edge_cases() {
    assert_eq!(nearest_group_10(65535, 0), 0);
    assert_eq!(nearest_group_10(1, 2), 0);
    assert_eq!(nearest_group_10(65535, 9), 10);
    assert_eq!(nearest_group_10(10, 11), 10);
    assert_eq!(nearest_group_10(65535, 11), 10);
    assert_eq!(nearest_group_10(65535, 65535), 65540);
    assert_eq!(nearest_group_10(65535, 20), 20);
    assert_eq!(nearest_group_10(40, 41), 40);
    assert_eq!(nearest_group_10(65535, 50), 50);
    assert_eq!(nearest_group_10(100, 101), 100);
    assert_eq!(nearest_group_10(65535, 101), 100);
    assert_eq!(nearest_group_10(255, 256), 260);
    assert_eq!(nearest_group_10(65535, 256), 260);
    assert_eq!(nearest_group_10(1, 0), 0);
    assert_eq!(nearest_group_10(0, 1), 0);
    assert_eq!(nearest_group_10(0, 0), 0);
    assert_eq!(nearest_group_10(1, 1), 0);
    assert_eq!(nearest_group_10(9, 9), 10);
    assert_eq!(nearest_group_10(10, 10), 10);
    assert_eq!(nearest_group_10(11, 11), 10);
    assert_eq!(nearest_group_10(12, 12), 10);
    assert_eq!(nearest_group_10(13, 13), 10);
    assert_eq!(nearest_group_10(20, 20), 20);
    assert_eq!(nearest_group_10(21, 21), 20);
}

#[test]
fn submission_marker() {
    assert_eq!(task_159::TRACK_PROOF, 0x52540000 | 159);
}
