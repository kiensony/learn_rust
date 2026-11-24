use task_148::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_place_8(0, 2), 0);
    assert_eq!(digit_place_8(1, 3), 0);
    assert_eq!(digit_place_8(7, 4), 0);
    assert_eq!(digit_place_8(8, 5), 0);
    assert_eq!(digit_place_8(9, 6), 0);
    assert_eq!(digit_place_8(65535, 0), 7);
    assert_eq!(digit_place_8(20, 1), 2);
    assert_eq!(digit_place_8(40, 2), 4);
    assert_eq!(digit_place_8(50, 3), 5);
    assert_eq!(digit_place_8(100, 4), 4);
    assert_eq!(digit_place_8(101, 5), 4);
    assert_eq!(digit_place_8(255, 6), 7);
    assert_eq!(digit_place_8(256, 0), 0);
    assert_eq!(digit_place_8(18, 3), 1);
    assert_eq!(digit_place_8(3, 18), 1);
    assert_eq!(digit_place_8(4, 4), 0);
    assert_eq!(digit_place_8(0, 0), 0);
    assert_eq!(digit_place_8(1, 0), 0);
    assert_eq!(digit_place_8(7, 0), 0);
    assert_eq!(digit_place_8(8, 0), 1);
    assert_eq!(digit_place_8(9, 0), 1);
    assert_eq!(digit_place_8(10, 0), 1);
    assert_eq!(digit_place_8(11, 0), 1);
    assert_eq!(digit_place_8(16, 0), 2);
    assert_eq!(digit_place_8(17, 0), 2);
    assert_eq!(digit_place_8(20, 0), 2);
    assert_eq!(digit_place_8(100, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_148::TRACK_PROOF, 0x52540000 | 148);
}
