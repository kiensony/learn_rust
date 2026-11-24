mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Press \"Enter\" to continue.\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_017::TRACK_PROOF, 0x52540000 | 17, "See the submission step in REVIEW.md");
}
