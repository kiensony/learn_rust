use task_1577::*;

#[test]
fn edge_cases() {
    assert_eq!(native_archive(""), "track_");
    assert_eq!(native_archive("a-b-c"), "track_a_b_c");
    assert_eq!(native_archive("already_ok"), "track_already_ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1577::TRACK_PROOF, 0x52540000 | 1577);
}
