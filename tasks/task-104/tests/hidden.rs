use task_104::*;

#[test]
fn edge_cases() {
    assert_eq!(loose_tiles_10(65535, 0), 0);
    assert_eq!(loose_tiles_10(1, 2), 1);
    assert_eq!(loose_tiles_10(65535, 9), 9);
    assert_eq!(loose_tiles_10(10, 11), 0);
    assert_eq!(loose_tiles_10(65535, 11), 1);
    assert_eq!(loose_tiles_10(65535, 65535), 5);
    assert_eq!(loose_tiles_10(1, 0), 0);
    assert_eq!(loose_tiles_10(0, 1), 0);
    assert_eq!(loose_tiles_10(0, 0), 0);
    assert_eq!(loose_tiles_10(1, 1), 1);
    assert_eq!(loose_tiles_10(9, 9), 9);
    assert_eq!(loose_tiles_10(10, 10), 0);
    assert_eq!(loose_tiles_10(11, 11), 1);
    assert_eq!(loose_tiles_10(12, 12), 2);
    assert_eq!(loose_tiles_10(13, 13), 3);
    assert_eq!(loose_tiles_10(20, 20), 0);
    assert_eq!(loose_tiles_10(21, 21), 1);
    assert_eq!(loose_tiles_10(100, 100), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_104::TRACK_PROOF, 0x52540000 | 104);
}
