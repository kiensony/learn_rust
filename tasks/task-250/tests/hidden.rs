use task_250::*;

#[test]
fn edge_cases() {
    assert_eq!(count_chimes_10(255, 0), 0);
    assert_eq!(count_chimes_10(1, 2), 0);
    assert_eq!(count_chimes_10(255, 9), 0);
    assert_eq!(count_chimes_10(10, 11), 1);
    assert_eq!(count_chimes_10(255, 11), 1);
    assert_eq!(count_chimes_10(255, 255), 25);
    assert_eq!(count_chimes_10(1, 0), 0);
    assert_eq!(count_chimes_10(0, 1), 0);
    assert_eq!(count_chimes_10(0, 0), 0);
    assert_eq!(count_chimes_10(1, 1), 0);
    assert_eq!(count_chimes_10(9, 9), 0);
    assert_eq!(count_chimes_10(10, 10), 1);
    assert_eq!(count_chimes_10(11, 11), 1);
    assert_eq!(count_chimes_10(12, 12), 1);
    assert_eq!(count_chimes_10(13, 13), 1);
    assert_eq!(count_chimes_10(20, 20), 2);
    assert_eq!(count_chimes_10(21, 21), 2);
    assert_eq!(count_chimes_10(100, 100), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_250::TRACK_PROOF, 0x52540000 | 250);
}
