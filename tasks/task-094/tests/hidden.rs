use task_094::*;

#[test]
fn edge_cases() {
    assert_eq!(loose_tiles_8(13, 1), 4);
    assert_eq!(loose_tiles_8(1, 3), 2);
    assert_eq!(loose_tiles_8(7, 4), 3);
    assert_eq!(loose_tiles_8(8, 5), 3);
    assert_eq!(loose_tiles_8(9, 6), 3);
    assert_eq!(loose_tiles_8(65535, 0), 7);
    assert_eq!(loose_tiles_8(18, 3), 7);
    assert_eq!(loose_tiles_8(3, 18), 7);
    assert_eq!(loose_tiles_8(4, 4), 0);
    assert_eq!(loose_tiles_8(0, 0), 0);
    assert_eq!(loose_tiles_8(1, 0), 1);
    assert_eq!(loose_tiles_8(7, 0), 7);
    assert_eq!(loose_tiles_8(8, 0), 0);
    assert_eq!(loose_tiles_8(9, 0), 1);
    assert_eq!(loose_tiles_8(10, 0), 2);
    assert_eq!(loose_tiles_8(11, 0), 3);
    assert_eq!(loose_tiles_8(16, 0), 0);
    assert_eq!(loose_tiles_8(17, 0), 1);
    assert_eq!(loose_tiles_8(20, 0), 4);
    assert_eq!(loose_tiles_8(100, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_094::TRACK_PROOF, 0x52540000 | 94);
}
