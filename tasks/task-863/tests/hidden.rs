use task_863::*;

#[test]
fn edge_cases() {
    assert_eq!(is_exit(Command::Write(0)), false);
    assert_eq!(is_exit(Command::Read), false);
    assert_eq!(is_exit(Command::Write(255)), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_863::TRACK_PROOF, 0x52540000 | 863);
}
