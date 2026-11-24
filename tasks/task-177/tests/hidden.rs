use task_177::*;

#[test]
fn edge_cases() {
    assert_eq!(three_shelves_3(0, 6), 2);
    assert_eq!(three_shelves_3(1, 9), 2);
    assert_eq!(three_shelves_3(2, 12), 2);
    assert_eq!(three_shelves_3(3, 15), 2);
    assert_eq!(three_shelves_3(4, 1), 1);
    assert_eq!(three_shelves_3(65535, 4), 2);
    assert_eq!(three_shelves_3(5, 7), 2);
    assert_eq!(three_shelves_3(6, 10), 2);
    assert_eq!(three_shelves_3(6, 13), 2);
    assert_eq!(three_shelves_3(7, 16), 2);
    assert_eq!(three_shelves_3(65535, 1), 2);
    assert_eq!(three_shelves_3(0, 0), 0);
    assert_eq!(three_shelves_3(1, 0), 0);
    assert_eq!(three_shelves_3(2, 0), 0);
    assert_eq!(three_shelves_3(3, 0), 1);
    assert_eq!(three_shelves_3(5, 0), 1);
    assert_eq!(three_shelves_3(6, 0), 2);
    assert_eq!(three_shelves_3(7, 0), 2);
    assert_eq!(three_shelves_3(20, 0), 2);
    assert_eq!(three_shelves_3(100, 0), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_177::TRACK_PROOF, 0x52540000 | 177);
}
