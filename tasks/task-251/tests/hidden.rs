use task_251::*;

#[test]
fn edge_cases() {
    assert_eq!(sum_chimes_10(255, 0), 0);
    assert_eq!(sum_chimes_10(1, 2), 0);
    assert_eq!(sum_chimes_10(255, 9), 0);
    assert_eq!(sum_chimes_10(10, 11), 10);
    assert_eq!(sum_chimes_10(255, 11), 10);
    assert_eq!(sum_chimes_10(255, 255), 3250);
    assert_eq!(sum_chimes_10(1, 0), 0);
    assert_eq!(sum_chimes_10(0, 1), 0);
    assert_eq!(sum_chimes_10(0, 0), 0);
    assert_eq!(sum_chimes_10(1, 1), 0);
    assert_eq!(sum_chimes_10(9, 9), 0);
    assert_eq!(sum_chimes_10(10, 10), 10);
    assert_eq!(sum_chimes_10(11, 11), 10);
    assert_eq!(sum_chimes_10(12, 12), 10);
    assert_eq!(sum_chimes_10(13, 13), 10);
    assert_eq!(sum_chimes_10(20, 20), 30);
    assert_eq!(sum_chimes_10(21, 21), 30);
    assert_eq!(sum_chimes_10(100, 100), 550);
}

#[test]
fn submission_marker() {
    assert_eq!(task_251::TRACK_PROOF, 0x52540000 | 251);
}
