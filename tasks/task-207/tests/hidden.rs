use task_207::*;

#[test]
fn edge_cases() {
    assert_eq!(three_shelves_10(65535, 0), 0);
    assert_eq!(three_shelves_10(1, 2), 0);
    assert_eq!(three_shelves_10(65535, 9), 0);
    assert_eq!(three_shelves_10(10, 11), 1);
    assert_eq!(three_shelves_10(65535, 11), 1);
    assert_eq!(three_shelves_10(65535, 65535), 2);
    assert_eq!(three_shelves_10(65535, 12), 1);
    assert_eq!(three_shelves_10(65535, 20), 2);
    assert_eq!(three_shelves_10(21, 22), 2);
    assert_eq!(three_shelves_10(1, 0), 0);
    assert_eq!(three_shelves_10(0, 1), 0);
    assert_eq!(three_shelves_10(0, 0), 0);
    assert_eq!(three_shelves_10(1, 1), 0);
    assert_eq!(three_shelves_10(9, 9), 0);
    assert_eq!(three_shelves_10(10, 10), 1);
    assert_eq!(three_shelves_10(11, 11), 1);
    assert_eq!(three_shelves_10(12, 12), 1);
    assert_eq!(three_shelves_10(13, 13), 2);
    assert_eq!(three_shelves_10(20, 20), 2);
    assert_eq!(three_shelves_10(21, 21), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_207::TRACK_PROOF, 0x52540000 | 207);
}
