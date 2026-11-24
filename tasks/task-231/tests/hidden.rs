use task_231::*;

#[test]
fn edge_cases() {
    assert_eq!(sum_chimes_5(0, 2), 0);
    assert_eq!(sum_chimes_5(1, 0), 0);
    assert_eq!(sum_chimes_5(4, 1), 0);
    assert_eq!(sum_chimes_5(5, 2), 0);
    assert_eq!(sum_chimes_5(6, 0), 5);
    assert_eq!(sum_chimes_5(255, 1), 6375);
    assert_eq!(sum_chimes_5(0, 9), 0);
    assert_eq!(sum_chimes_5(100, 0), 1050);
    assert_eq!(sum_chimes_5(100, 1), 950);
    assert_eq!(sum_chimes_5(0, 0), 0);
    assert_eq!(sum_chimes_5(5, 0), 5);
    assert_eq!(sum_chimes_5(7, 0), 5);
    assert_eq!(sum_chimes_5(8, 0), 5);
    assert_eq!(sum_chimes_5(10, 0), 15);
    assert_eq!(sum_chimes_5(11, 0), 15);
    assert_eq!(sum_chimes_5(20, 0), 50);
}

#[test]
fn submission_marker() {
    assert_eq!(task_231::TRACK_PROOF, 0x52540000 | 231);
}
