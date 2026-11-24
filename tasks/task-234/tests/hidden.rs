use task_234::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_sum_5(13, 1), 4);
    assert_eq!(digit_sum_5(1, 0), 1);
    assert_eq!(digit_sum_5(4, 1), 3);
    assert_eq!(digit_sum_5(5, 2), 3);
    assert_eq!(digit_sum_5(6, 0), 2);
    assert_eq!(digit_sum_5(255, 1), 6);
    assert_eq!(digit_sum_5(0, 9), 0);
    assert_eq!(digit_sum_5(100, 0), 4);
    assert_eq!(digit_sum_5(100, 1), 11);
    assert_eq!(digit_sum_5(0, 0), 0);
    assert_eq!(digit_sum_5(5, 0), 1);
    assert_eq!(digit_sum_5(7, 0), 3);
    assert_eq!(digit_sum_5(8, 0), 4);
    assert_eq!(digit_sum_5(10, 0), 2);
    assert_eq!(digit_sum_5(11, 0), 3);
    assert_eq!(digit_sum_5(20, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_234::TRACK_PROOF, 0x52540000 | 234);
}
