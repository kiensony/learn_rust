use task_230::*;

#[test]
fn edge_cases() {
    assert_eq!(count_chimes_5(0, 2), 0);
    assert_eq!(count_chimes_5(1, 0), 0);
    assert_eq!(count_chimes_5(4, 1), 0);
    assert_eq!(count_chimes_5(5, 2), 0);
    assert_eq!(count_chimes_5(6, 0), 1);
    assert_eq!(count_chimes_5(255, 1), 50);
    assert_eq!(count_chimes_5(0, 9), 0);
    assert_eq!(count_chimes_5(100, 0), 20);
    assert_eq!(count_chimes_5(100, 1), 19);
    assert_eq!(count_chimes_5(0, 0), 0);
    assert_eq!(count_chimes_5(5, 0), 1);
    assert_eq!(count_chimes_5(7, 0), 1);
    assert_eq!(count_chimes_5(8, 0), 1);
    assert_eq!(count_chimes_5(10, 0), 2);
    assert_eq!(count_chimes_5(11, 0), 2);
    assert_eq!(count_chimes_5(20, 0), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_230::TRACK_PROOF, 0x52540000 | 230);
}
