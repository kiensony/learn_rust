use task_138::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_place_5(0, 2), 0);
    assert_eq!(digit_place_5(1, 0), 0);
    assert_eq!(digit_place_5(4, 1), 0);
    assert_eq!(digit_place_5(5, 2), 0);
    assert_eq!(digit_place_5(6, 0), 1);
    assert_eq!(digit_place_5(65535, 1), 1);
    assert_eq!(digit_place_5(20, 2), 3);
    assert_eq!(digit_place_5(40, 0), 3);
    assert_eq!(digit_place_5(50, 1), 4);
    assert_eq!(digit_place_5(100, 2), 4);
    assert_eq!(digit_place_5(101, 0), 0);
    assert_eq!(digit_place_5(255, 1), 0);
    assert_eq!(digit_place_5(256, 2), 0);
    assert_eq!(digit_place_5(0, 9), 0);
    assert_eq!(digit_place_5(100, 0), 0);
    assert_eq!(digit_place_5(100, 1), 4);
    assert_eq!(digit_place_5(0, 0), 0);
    assert_eq!(digit_place_5(5, 0), 1);
    assert_eq!(digit_place_5(7, 0), 1);
    assert_eq!(digit_place_5(8, 0), 1);
    assert_eq!(digit_place_5(10, 0), 2);
    assert_eq!(digit_place_5(11, 0), 2);
    assert_eq!(digit_place_5(20, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_138::TRACK_PROOF, 0x52540000 | 138);
}
