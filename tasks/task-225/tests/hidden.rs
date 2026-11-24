use task_225::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_count_3(0, 6), 2);
    assert_eq!(digit_count_3(1, 9), 3);
    assert_eq!(digit_count_3(2, 12), 3);
    assert_eq!(digit_count_3(3, 15), 3);
    assert_eq!(digit_count_3(4, 1), 2);
    assert_eq!(digit_count_3(255, 4), 6);
    assert_eq!(digit_count_3(255, 1), 6);
    assert_eq!(digit_count_3(0, 0), 1);
    assert_eq!(digit_count_3(1, 0), 1);
    assert_eq!(digit_count_3(2, 0), 1);
    assert_eq!(digit_count_3(3, 0), 2);
    assert_eq!(digit_count_3(5, 0), 2);
    assert_eq!(digit_count_3(6, 0), 2);
    assert_eq!(digit_count_3(7, 0), 2);
    assert_eq!(digit_count_3(20, 0), 3);
    assert_eq!(digit_count_3(100, 0), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_225::TRACK_PROOF, 0x52540000 | 225);
}
