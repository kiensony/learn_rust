mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "gate closed\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_045::TRACK_PROOF, 0x52540000 | 45, "See the submission step in REVIEW.md");
}
