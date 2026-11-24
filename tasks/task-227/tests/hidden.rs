use task_227::*;

#[test]
fn edge_cases() {
    assert_eq!(cycle_ticks_3(0, 6), 3);
    assert_eq!(cycle_ticks_3(1, 9), 0);
    assert_eq!(cycle_ticks_3(2, 12), 2);
    assert_eq!(cycle_ticks_3(3, 15), 4);
    assert_eq!(cycle_ticks_3(4, 1), 0);
    assert_eq!(cycle_ticks_3(255, 4), 0);
    assert_eq!(cycle_ticks_3(255, 1), 0);
    assert_eq!(cycle_ticks_3(0, 0), 0);
    assert_eq!(cycle_ticks_3(1, 0), 3);
    assert_eq!(cycle_ticks_3(2, 0), 1);
    assert_eq!(cycle_ticks_3(3, 0), 4);
    assert_eq!(cycle_ticks_3(5, 0), 0);
    assert_eq!(cycle_ticks_3(6, 0), 3);
    assert_eq!(cycle_ticks_3(7, 0), 1);
    assert_eq!(cycle_ticks_3(20, 0), 0);
    assert_eq!(cycle_ticks_3(100, 0), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_227::TRACK_PROOF, 0x52540000 | 227);
}
