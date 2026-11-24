use task_215::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_count_2(0), 1);
    assert_eq!(digit_count_2(1), 1);
    assert_eq!(digit_count_2(1), 1);
    assert_eq!(digit_count_2(2), 2);
    assert_eq!(digit_count_2(3), 2);
    assert_eq!(digit_count_2(255), 8);
}

#[test]
fn submission_marker() {
    assert_eq!(task_215::TRACK_PROOF, 0x52540000 | 215);
}
