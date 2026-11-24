use task_216::*;

#[test]
fn edge_cases() {
    assert_eq!(doubling_steps_2(0), 0);
    assert_eq!(doubling_steps_2(1), 0);
    assert_eq!(doubling_steps_2(1), 0);
    assert_eq!(doubling_steps_2(2), 0);
    assert_eq!(doubling_steps_2(3), 1);
    assert_eq!(doubling_steps_2(255), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_216::TRACK_PROOF, 0x52540000 | 216);
}
