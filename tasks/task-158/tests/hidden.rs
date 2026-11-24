use task_158::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_place_10(65535, 0), 0);
    assert_eq!(digit_place_10(1, 2), 0);
    assert_eq!(digit_place_10(65535, 9), 0);
    assert_eq!(digit_place_10(10, 11), 1);
    assert_eq!(digit_place_10(65535, 11), 1);
    assert_eq!(digit_place_10(65535, 65535), 3);
    assert_eq!(digit_place_10(65535, 20), 2);
    assert_eq!(digit_place_10(40, 41), 4);
    assert_eq!(digit_place_10(65535, 50), 5);
    assert_eq!(digit_place_10(100, 101), 0);
    assert_eq!(digit_place_10(65535, 101), 0);
    assert_eq!(digit_place_10(255, 256), 5);
    assert_eq!(digit_place_10(65535, 256), 5);
    assert_eq!(digit_place_10(1, 0), 0);
    assert_eq!(digit_place_10(0, 1), 0);
    assert_eq!(digit_place_10(0, 0), 0);
    assert_eq!(digit_place_10(1, 1), 0);
    assert_eq!(digit_place_10(9, 9), 0);
    assert_eq!(digit_place_10(10, 10), 1);
    assert_eq!(digit_place_10(11, 11), 1);
    assert_eq!(digit_place_10(12, 12), 1);
    assert_eq!(digit_place_10(13, 13), 1);
    assert_eq!(digit_place_10(20, 20), 2);
    assert_eq!(digit_place_10(21, 21), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_158::TRACK_PROOF, 0x52540000 | 158);
}
