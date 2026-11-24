mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Name> ", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_024::TRACK_PROOF, 0x52540000 | 24, "See the submission step in REVIEW.md");
}
