use task_217::*;

#[test]
fn edge_cases() {
    assert_eq!(cycle_ticks_2(0), 0);
    assert_eq!(cycle_ticks_2(1), 2);
    assert_eq!(cycle_ticks_2(1), 2);
    assert_eq!(cycle_ticks_2(2), 0);
    assert_eq!(cycle_ticks_2(3), 2);
    assert_eq!(cycle_ticks_2(255), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_217::TRACK_PROOF, 0x52540000 | 217);
}
