mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "open {file}\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_022::TRACK_PROOF, 0x52540000 | 22, "See the submission step in REVIEW.md");
}
