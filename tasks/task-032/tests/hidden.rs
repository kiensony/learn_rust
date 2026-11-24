mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "package task_032\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_032::TRACK_PROOF, 0x52540000 | 32, "See the submission step in REVIEW.md");
}
