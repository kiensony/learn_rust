use task_254::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_sum_10(13, 14), 4);
    assert_eq!(digit_sum_10(1, 2), 1);
    assert_eq!(digit_sum_10(255, 9), 9);
    assert_eq!(digit_sum_10(10, 11), 1);
    assert_eq!(digit_sum_10(255, 11), 2);
    assert_eq!(digit_sum_10(255, 255), 12);
    assert_eq!(digit_sum_10(1, 0), 0);
    assert_eq!(digit_sum_10(0, 1), 0);
    assert_eq!(digit_sum_10(0, 0), 0);
    assert_eq!(digit_sum_10(1, 1), 1);
    assert_eq!(digit_sum_10(9, 9), 9);
    assert_eq!(digit_sum_10(10, 10), 1);
    assert_eq!(digit_sum_10(11, 11), 2);
    assert_eq!(digit_sum_10(12, 12), 3);
    assert_eq!(digit_sum_10(13, 13), 4);
    assert_eq!(digit_sum_10(20, 20), 2);
    assert_eq!(digit_sum_10(21, 21), 3);
    assert_eq!(digit_sum_10(100, 100), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_254::TRACK_PROOF, 0x52540000 | 254);
}
