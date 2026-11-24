use task_245::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_count_8(0, 2), 1);
    assert_eq!(digit_count_8(1, 3), 1);
    assert_eq!(digit_count_8(7, 4), 1);
    assert_eq!(digit_count_8(8, 5), 1);
    assert_eq!(digit_count_8(9, 6), 1);
    assert_eq!(digit_count_8(255, 0), 3);
    assert_eq!(digit_count_8(18, 3), 2);
    assert_eq!(digit_count_8(3, 18), 2);
    assert_eq!(digit_count_8(4, 4), 1);
    assert_eq!(digit_count_8(0, 0), 1);
    assert_eq!(digit_count_8(1, 0), 1);
    assert_eq!(digit_count_8(7, 0), 1);
    assert_eq!(digit_count_8(8, 0), 2);
    assert_eq!(digit_count_8(9, 0), 2);
    assert_eq!(digit_count_8(10, 0), 2);
    assert_eq!(digit_count_8(11, 0), 2);
    assert_eq!(digit_count_8(16, 0), 2);
    assert_eq!(digit_count_8(17, 0), 2);
    assert_eq!(digit_count_8(20, 0), 2);
    assert_eq!(digit_count_8(100, 0), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_245::TRACK_PROOF, 0x52540000 | 245);
}
