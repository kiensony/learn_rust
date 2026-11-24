use task_257::*;

#[test]
fn edge_cases() {
    assert_eq!(cycle_ticks_10(255, 0), 0);
    assert_eq!(cycle_ticks_10(1, 2), 10);
    assert_eq!(cycle_ticks_10(255, 9), 6);
    assert_eq!(cycle_ticks_10(10, 11), 4);
    assert_eq!(cycle_ticks_10(255, 11), 2);
    assert_eq!(cycle_ticks_10(255, 255), 6);
    assert_eq!(cycle_ticks_10(1, 0), 0);
    assert_eq!(cycle_ticks_10(0, 1), 0);
    assert_eq!(cycle_ticks_10(0, 0), 0);
    assert_eq!(cycle_ticks_10(1, 1), 10);
    assert_eq!(cycle_ticks_10(9, 9), 6);
    assert_eq!(cycle_ticks_10(10, 10), 4);
    assert_eq!(cycle_ticks_10(11, 11), 2);
    assert_eq!(cycle_ticks_10(12, 12), 0);
    assert_eq!(cycle_ticks_10(13, 13), 10);
    assert_eq!(cycle_ticks_10(20, 20), 8);
    assert_eq!(cycle_ticks_10(21, 21), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_257::TRACK_PROOF, 0x52540000 | 257);
}
