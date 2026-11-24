use task_241::*;

#[test]
fn edge_cases() {
    assert_eq!(sum_chimes_8(0, 2), 0);
    assert_eq!(sum_chimes_8(1, 3), 0);
    assert_eq!(sum_chimes_8(7, 4), 0);
    assert_eq!(sum_chimes_8(8, 5), 0);
    assert_eq!(sum_chimes_8(9, 6), 0);
    assert_eq!(sum_chimes_8(255, 0), 3968);
    assert_eq!(sum_chimes_8(18, 3), 8);
    assert_eq!(sum_chimes_8(3, 18), 8);
    assert_eq!(sum_chimes_8(4, 4), 0);
    assert_eq!(sum_chimes_8(0, 0), 0);
    assert_eq!(sum_chimes_8(1, 0), 0);
    assert_eq!(sum_chimes_8(7, 0), 0);
    assert_eq!(sum_chimes_8(8, 0), 8);
    assert_eq!(sum_chimes_8(9, 0), 8);
    assert_eq!(sum_chimes_8(10, 0), 8);
    assert_eq!(sum_chimes_8(11, 0), 8);
    assert_eq!(sum_chimes_8(16, 0), 24);
    assert_eq!(sum_chimes_8(17, 0), 24);
    assert_eq!(sum_chimes_8(20, 0), 24);
    assert_eq!(sum_chimes_8(100, 0), 624);
}

#[test]
fn submission_marker() {
    assert_eq!(task_241::TRACK_PROOF, 0x52540000 | 241);
}
