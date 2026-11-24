use task_197::*;

#[test]
fn edge_cases() {
    assert_eq!(three_shelves_8(0, 2), 0);
    assert_eq!(three_shelves_8(1, 3), 0);
    assert_eq!(three_shelves_8(7, 4), 0);
    assert_eq!(three_shelves_8(8, 5), 0);
    assert_eq!(three_shelves_8(9, 6), 0);
    assert_eq!(three_shelves_8(65535, 0), 2);
    assert_eq!(three_shelves_8(10, 1), 1);
    assert_eq!(three_shelves_8(11, 2), 1);
    assert_eq!(three_shelves_8(16, 3), 2);
    assert_eq!(three_shelves_8(17, 4), 2);
    assert_eq!(three_shelves_8(18, 3), 2);
    assert_eq!(three_shelves_8(3, 18), 2);
    assert_eq!(three_shelves_8(4, 4), 0);
    assert_eq!(three_shelves_8(0, 0), 0);
    assert_eq!(three_shelves_8(1, 0), 0);
    assert_eq!(three_shelves_8(7, 0), 0);
    assert_eq!(three_shelves_8(8, 0), 1);
    assert_eq!(three_shelves_8(9, 0), 1);
    assert_eq!(three_shelves_8(10, 0), 1);
    assert_eq!(three_shelves_8(11, 0), 2);
    assert_eq!(three_shelves_8(16, 0), 2);
    assert_eq!(three_shelves_8(17, 0), 2);
    assert_eq!(three_shelves_8(20, 0), 2);
    assert_eq!(three_shelves_8(100, 0), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_197::TRACK_PROOF, 0x52540000 | 197);
}
