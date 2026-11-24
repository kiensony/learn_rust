mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "ready\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_038::TRACK_PROOF, 0x52540000 | 38, "See the submission step in REVIEW.md");
}
