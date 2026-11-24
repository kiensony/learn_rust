use task_084::*;

#[test]
fn edge_cases() {
    assert_eq!(loose_tiles_5(0, 2), 0);
    assert_eq!(loose_tiles_5(1, 0), 1);
    assert_eq!(loose_tiles_5(4, 1), 3);
    assert_eq!(loose_tiles_5(5, 2), 3);
    assert_eq!(loose_tiles_5(6, 0), 1);
    assert_eq!(loose_tiles_5(65535, 1), 4);
    assert_eq!(loose_tiles_5(0, 9), 0);
    assert_eq!(loose_tiles_5(100, 0), 0);
    assert_eq!(loose_tiles_5(100, 1), 4);
    assert_eq!(loose_tiles_5(0, 0), 0);
    assert_eq!(loose_tiles_5(5, 0), 0);
    assert_eq!(loose_tiles_5(7, 0), 2);
    assert_eq!(loose_tiles_5(8, 0), 3);
    assert_eq!(loose_tiles_5(10, 0), 0);
    assert_eq!(loose_tiles_5(11, 0), 1);
    assert_eq!(loose_tiles_5(20, 0), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_084::TRACK_PROOF, 0x52540000 | 84);
}
