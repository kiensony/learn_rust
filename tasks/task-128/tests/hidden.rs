use task_128::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_place_3(0, 6), 2);
    assert_eq!(digit_place_3(1, 9), 0);
    assert_eq!(digit_place_3(2, 12), 1);
    assert_eq!(digit_place_3(3, 15), 0);
    assert_eq!(digit_place_3(4, 1), 1);
    assert_eq!(digit_place_3(65535, 4), 2);
    assert_eq!(digit_place_3(20, 7), 0);
    assert_eq!(digit_place_3(40, 10), 1);
    assert_eq!(digit_place_3(50, 13), 0);
    assert_eq!(digit_place_3(100, 16), 2);
    assert_eq!(digit_place_3(101, 2), 1);
    assert_eq!(digit_place_3(255, 5), 2);
    assert_eq!(digit_place_3(256, 8), 1);
    assert_eq!(digit_place_3(65535, 1), 2);
    assert_eq!(digit_place_3(0, 0), 0);
    assert_eq!(digit_place_3(1, 0), 0);
    assert_eq!(digit_place_3(2, 0), 0);
    assert_eq!(digit_place_3(3, 0), 1);
    assert_eq!(digit_place_3(5, 0), 1);
    assert_eq!(digit_place_3(6, 0), 2);
    assert_eq!(digit_place_3(7, 0), 2);
    assert_eq!(digit_place_3(20, 0), 0);
    assert_eq!(digit_place_3(100, 0), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_128::TRACK_PROOF, 0x52540000 | 128);
}
