use task_074::*;

#[test]
fn edge_cases() {
    assert_eq!(loose_tiles_3(13, 3), 1);
    assert_eq!(loose_tiles_3(1, 9), 1);
    assert_eq!(loose_tiles_3(2, 12), 2);
    assert_eq!(loose_tiles_3(3, 15), 0);
    assert_eq!(loose_tiles_3(4, 1), 2);
    assert_eq!(loose_tiles_3(65535, 4), 0);
    assert_eq!(loose_tiles_3(65535, 1), 0);
    assert_eq!(loose_tiles_3(0, 0), 0);
    assert_eq!(loose_tiles_3(1, 0), 1);
    assert_eq!(loose_tiles_3(2, 0), 2);
    assert_eq!(loose_tiles_3(3, 0), 0);
    assert_eq!(loose_tiles_3(5, 0), 2);
    assert_eq!(loose_tiles_3(6, 0), 0);
    assert_eq!(loose_tiles_3(7, 0), 1);
    assert_eq!(loose_tiles_3(20, 0), 2);
    assert_eq!(loose_tiles_3(100, 0), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_074::TRACK_PROOF, 0x52540000 | 74);
}
