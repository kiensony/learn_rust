use task_097::*;

#[test]
fn edge_cases() {
    assert_eq!(reverse_a_reading_8(0, 2), 8191);
    assert_eq!(reverse_a_reading_8(1, 3), 8191);
    assert_eq!(reverse_a_reading_8(7, 4), 8191);
    assert_eq!(reverse_a_reading_8(8, 5), 8191);
    assert_eq!(reverse_a_reading_8(9, 6), 8191);
    assert_eq!(reverse_a_reading_8(65535, 0), 0);
    assert_eq!(reverse_a_reading_8(18, 3), 8190);
    assert_eq!(reverse_a_reading_8(3, 18), 8190);
    assert_eq!(reverse_a_reading_8(4, 4), 8191);
    assert_eq!(reverse_a_reading_8(0, 0), 8191);
    assert_eq!(reverse_a_reading_8(1, 0), 8191);
    assert_eq!(reverse_a_reading_8(7, 0), 8191);
    assert_eq!(reverse_a_reading_8(8, 0), 8190);
    assert_eq!(reverse_a_reading_8(9, 0), 8190);
    assert_eq!(reverse_a_reading_8(10, 0), 8190);
    assert_eq!(reverse_a_reading_8(11, 0), 8190);
    assert_eq!(reverse_a_reading_8(16, 0), 8189);
    assert_eq!(reverse_a_reading_8(17, 0), 8189);
    assert_eq!(reverse_a_reading_8(20, 0), 8189);
    assert_eq!(reverse_a_reading_8(100, 0), 8179);
}

#[test]
fn submission_marker() {
    assert_eq!(task_097::TRACK_PROOF, 0x52540000 | 97);
}
