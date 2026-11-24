use task_224::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_sum_3(0, 6), 2);
    assert_eq!(digit_sum_3(1, 9), 2);
    assert_eq!(digit_sum_3(2, 12), 4);
    assert_eq!(digit_sum_3(3, 15), 2);
    assert_eq!(digit_sum_3(4, 1), 3);
    assert_eq!(digit_sum_3(255, 4), 3);
    assert_eq!(digit_sum_3(255, 1), 3);
    assert_eq!(digit_sum_3(0, 0), 0);
    assert_eq!(digit_sum_3(1, 0), 1);
    assert_eq!(digit_sum_3(2, 0), 2);
    assert_eq!(digit_sum_3(3, 0), 1);
    assert_eq!(digit_sum_3(5, 0), 3);
    assert_eq!(digit_sum_3(6, 0), 2);
    assert_eq!(digit_sum_3(7, 0), 3);
    assert_eq!(digit_sum_3(20, 0), 4);
    assert_eq!(digit_sum_3(100, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_224::TRACK_PROOF, 0x52540000 | 224);
}
