use task_064::*;

#[test]
fn edge_cases() {
    assert_eq!(loose_tiles_2(0), 0);
    assert_eq!(loose_tiles_2(1), 1);
    assert_eq!(loose_tiles_2(1), 1);
    assert_eq!(loose_tiles_2(2), 0);
    assert_eq!(loose_tiles_2(3), 1);
    assert_eq!(loose_tiles_2(65535), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_064::TRACK_PROOF, 0x52540000 | 64);
}
