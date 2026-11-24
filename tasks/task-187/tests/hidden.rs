use task_187::*;

#[test]
fn edge_cases() {
    assert_eq!(three_shelves_5(0, 2), 0);
    assert_eq!(three_shelves_5(1, 0), 0);
    assert_eq!(three_shelves_5(4, 1), 0);
    assert_eq!(three_shelves_5(5, 2), 0);
    assert_eq!(three_shelves_5(6, 0), 1);
    assert_eq!(three_shelves_5(65535, 1), 2);
    assert_eq!(three_shelves_5(7, 2), 1);
    assert_eq!(three_shelves_5(8, 0), 2);
    assert_eq!(three_shelves_5(10, 1), 2);
    assert_eq!(three_shelves_5(11, 2), 2);
    assert_eq!(three_shelves_5(0, 9), 0);
    assert_eq!(three_shelves_5(100, 0), 2);
    assert_eq!(three_shelves_5(100, 1), 2);
    assert_eq!(three_shelves_5(0, 0), 0);
    assert_eq!(three_shelves_5(5, 0), 1);
    assert_eq!(three_shelves_5(7, 0), 1);
    assert_eq!(three_shelves_5(10, 0), 2);
    assert_eq!(three_shelves_5(11, 0), 2);
    assert_eq!(three_shelves_5(20, 0), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_187::TRACK_PROOF, 0x52540000 | 187);
}
