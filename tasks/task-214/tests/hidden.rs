use task_214::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_sum_2(0), 0);
    assert_eq!(digit_sum_2(1), 1);
    assert_eq!(digit_sum_2(1), 1);
    assert_eq!(digit_sum_2(2), 1);
    assert_eq!(digit_sum_2(3), 2);
    assert_eq!(digit_sum_2(255), 8);
}

#[test]
fn submission_marker() {
    assert_eq!(task_214::TRACK_PROOF, 0x52540000 | 214);
}
