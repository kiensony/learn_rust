use task_221::*;

#[test]
fn edge_cases() {
    assert_eq!(sum_chimes_3(0, 6), 9);
    assert_eq!(sum_chimes_3(1, 9), 18);
    assert_eq!(sum_chimes_3(2, 12), 30);
    assert_eq!(sum_chimes_3(3, 15), 63);
    assert_eq!(sum_chimes_3(4, 1), 3);
    assert_eq!(sum_chimes_3(255, 4), 10965);
    assert_eq!(sum_chimes_3(255, 1), 10965);
    assert_eq!(sum_chimes_3(0, 0), 0);
    assert_eq!(sum_chimes_3(1, 0), 0);
    assert_eq!(sum_chimes_3(2, 0), 0);
    assert_eq!(sum_chimes_3(3, 0), 3);
    assert_eq!(sum_chimes_3(5, 0), 3);
    assert_eq!(sum_chimes_3(6, 0), 9);
    assert_eq!(sum_chimes_3(7, 0), 9);
    assert_eq!(sum_chimes_3(20, 0), 63);
    assert_eq!(sum_chimes_3(100, 0), 1683);
}

#[test]
fn submission_marker() {
    assert_eq!(task_221::TRACK_PROOF, 0x52540000 | 221);
}
