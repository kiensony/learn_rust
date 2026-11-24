mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "items=3\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_039::TRACK_PROOF, 0x52540000 | 39, "See the submission step in REVIEW.md");
}
