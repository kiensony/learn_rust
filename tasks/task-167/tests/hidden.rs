use task_167::*;

#[test]
fn edge_cases() {
    assert_eq!(three_shelves_2(0), 0);
    assert_eq!(three_shelves_2(1), 0);
    assert_eq!(three_shelves_2(1), 0);
    assert_eq!(three_shelves_2(2), 1);
    assert_eq!(three_shelves_2(3), 1);
    assert_eq!(three_shelves_2(65535), 2);
    assert_eq!(three_shelves_2(4), 1);
    assert_eq!(three_shelves_2(5), 2);
    assert_eq!(three_shelves_2(4), 1);
    assert_eq!(three_shelves_2(5), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_167::TRACK_PROOF, 0x52540000 | 167);
}
