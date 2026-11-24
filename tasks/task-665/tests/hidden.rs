use task_665::*;

#[test]
fn edge_cases() {
    assert_eq!(wait_seconds(Signal::Closed), u16::MAX);
    assert_eq!(wait_seconds(Signal::Wait(0)), 0);
    assert_eq!(wait_seconds(Signal::Wait(65535)), 65535);
}

#[test]
fn submission_marker() {
    assert_eq!(task_665::TRACK_PROOF, 0x52540000 | 665);
}
