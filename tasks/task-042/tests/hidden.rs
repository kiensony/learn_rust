mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "---\nNew timetable\n---\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_042::TRACK_PROOF, 0x52540000 | 42, "See the submission step in REVIEW.md");
}
