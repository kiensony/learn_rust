mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "C:\\course\\notes\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_018::TRACK_PROOF, 0x52540000 | 18, "See the submission step in REVIEW.md");
}
