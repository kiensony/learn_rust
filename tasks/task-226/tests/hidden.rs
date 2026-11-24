use task_226::*;

#[test]
fn edge_cases() {
    assert_eq!(doubling_steps_3(0, 6), 1);
    assert_eq!(doubling_steps_3(1, 9), 2);
    assert_eq!(doubling_steps_3(2, 12), 3);
    assert_eq!(doubling_steps_3(3, 15), 3);
    assert_eq!(doubling_steps_3(4, 1), 1);
    assert_eq!(doubling_steps_3(255, 4), 7);
    assert_eq!(doubling_steps_3(255, 1), 7);
    assert_eq!(doubling_steps_3(0, 0), 0);
    assert_eq!(doubling_steps_3(1, 0), 0);
    assert_eq!(doubling_steps_3(2, 0), 0);
    assert_eq!(doubling_steps_3(3, 0), 0);
    assert_eq!(doubling_steps_3(5, 0), 1);
    assert_eq!(doubling_steps_3(6, 0), 1);
    assert_eq!(doubling_steps_3(7, 0), 2);
    assert_eq!(doubling_steps_3(20, 0), 3);
    assert_eq!(doubling_steps_3(100, 0), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_226::TRACK_PROOF, 0x52540000 | 226);
}
