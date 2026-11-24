use task_237::*;

#[test]
fn edge_cases() {
    assert_eq!(cycle_ticks_5(0, 2), 0);
    assert_eq!(cycle_ticks_5(1, 0), 5);
    assert_eq!(cycle_ticks_5(4, 1), 1);
    assert_eq!(cycle_ticks_5(5, 2), 1);
    assert_eq!(cycle_ticks_5(6, 0), 2);
    assert_eq!(cycle_ticks_5(255, 1), 3);
    assert_eq!(cycle_ticks_5(0, 9), 0);
    assert_eq!(cycle_ticks_5(100, 0), 3);
    assert_eq!(cycle_ticks_5(100, 1), 5);
    assert_eq!(cycle_ticks_5(0, 0), 0);
    assert_eq!(cycle_ticks_5(5, 0), 4);
    assert_eq!(cycle_ticks_5(7, 0), 0);
    assert_eq!(cycle_ticks_5(8, 0), 5);
    assert_eq!(cycle_ticks_5(10, 0), 1);
    assert_eq!(cycle_ticks_5(11, 0), 6);
    assert_eq!(cycle_ticks_5(20, 0), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_237::TRACK_PROOF, 0x52540000 | 237);
}
