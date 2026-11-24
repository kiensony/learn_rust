use task_247::*;

#[test]
fn edge_cases() {
    assert_eq!(cycle_ticks_8(0, 2), 6);
    assert_eq!(cycle_ticks_8(1, 3), 6);
    assert_eq!(cycle_ticks_8(7, 4), 4);
    assert_eq!(cycle_ticks_8(8, 5), 4);
    assert_eq!(cycle_ticks_8(9, 6), 4);
    assert_eq!(cycle_ticks_8(255, 0), 0);
    assert_eq!(cycle_ticks_8(18, 3), 0);
    assert_eq!(cycle_ticks_8(3, 18), 0);
    assert_eq!(cycle_ticks_8(4, 4), 0);
    assert_eq!(cycle_ticks_8(0, 0), 0);
    assert_eq!(cycle_ticks_8(1, 0), 8);
    assert_eq!(cycle_ticks_8(7, 0), 6);
    assert_eq!(cycle_ticks_8(8, 0), 4);
    assert_eq!(cycle_ticks_8(9, 0), 2);
    assert_eq!(cycle_ticks_8(10, 0), 0);
    assert_eq!(cycle_ticks_8(11, 0), 8);
    assert_eq!(cycle_ticks_8(16, 0), 8);
    assert_eq!(cycle_ticks_8(17, 0), 6);
    assert_eq!(cycle_ticks_8(20, 0), 0);
    assert_eq!(cycle_ticks_8(100, 0), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_247::TRACK_PROOF, 0x52540000 | 247);
}
