mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "begin\nend\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_019::TRACK_PROOF, 0x52540000 | 19, "See the submission step in REVIEW.md");
}
