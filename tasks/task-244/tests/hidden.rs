use task_244::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_sum_8(0, 2), 2);
    assert_eq!(digit_sum_8(1, 3), 2);
    assert_eq!(digit_sum_8(7, 4), 3);
    assert_eq!(digit_sum_8(8, 5), 3);
    assert_eq!(digit_sum_8(9, 6), 3);
    assert_eq!(digit_sum_8(255, 0), 17);
    assert_eq!(digit_sum_8(18, 3), 8);
    assert_eq!(digit_sum_8(3, 18), 8);
    assert_eq!(digit_sum_8(4, 4), 0);
    assert_eq!(digit_sum_8(0, 0), 0);
    assert_eq!(digit_sum_8(1, 0), 1);
    assert_eq!(digit_sum_8(7, 0), 7);
    assert_eq!(digit_sum_8(8, 0), 1);
    assert_eq!(digit_sum_8(9, 0), 2);
    assert_eq!(digit_sum_8(10, 0), 3);
    assert_eq!(digit_sum_8(11, 0), 4);
    assert_eq!(digit_sum_8(16, 0), 2);
    assert_eq!(digit_sum_8(17, 0), 3);
    assert_eq!(digit_sum_8(20, 0), 6);
    assert_eq!(digit_sum_8(100, 0), 9);
}

#[test]
fn submission_marker() {
    assert_eq!(task_244::TRACK_PROOF, 0x52540000 | 244);
}
