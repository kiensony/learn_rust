use task_235::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_count_5(0, 2), 1);
    assert_eq!(digit_count_5(1, 0), 1);
    assert_eq!(digit_count_5(4, 1), 1);
    assert_eq!(digit_count_5(5, 2), 1);
    assert_eq!(digit_count_5(6, 0), 2);
    assert_eq!(digit_count_5(255, 1), 4);
    assert_eq!(digit_count_5(0, 9), 1);
    assert_eq!(digit_count_5(100, 0), 3);
    assert_eq!(digit_count_5(100, 1), 3);
    assert_eq!(digit_count_5(0, 0), 1);
    assert_eq!(digit_count_5(5, 0), 2);
    assert_eq!(digit_count_5(7, 0), 2);
    assert_eq!(digit_count_5(8, 0), 2);
    assert_eq!(digit_count_5(10, 0), 2);
    assert_eq!(digit_count_5(11, 0), 2);
    assert_eq!(digit_count_5(20, 0), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_235::TRACK_PROOF, 0x52540000 | 235);
}
