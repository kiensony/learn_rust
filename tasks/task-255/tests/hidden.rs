use task_255::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_count_10(255, 0), 1);
    assert_eq!(digit_count_10(1, 2), 1);
    assert_eq!(digit_count_10(255, 9), 1);
    assert_eq!(digit_count_10(10, 11), 2);
    assert_eq!(digit_count_10(255, 11), 2);
    assert_eq!(digit_count_10(255, 255), 3);
    assert_eq!(digit_count_10(1, 0), 1);
    assert_eq!(digit_count_10(0, 1), 1);
    assert_eq!(digit_count_10(0, 0), 1);
    assert_eq!(digit_count_10(1, 1), 1);
    assert_eq!(digit_count_10(9, 9), 1);
    assert_eq!(digit_count_10(10, 10), 2);
    assert_eq!(digit_count_10(11, 11), 2);
    assert_eq!(digit_count_10(12, 12), 2);
    assert_eq!(digit_count_10(13, 13), 2);
    assert_eq!(digit_count_10(20, 20), 2);
    assert_eq!(digit_count_10(21, 21), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_255::TRACK_PROOF, 0x52540000 | 255);
}
