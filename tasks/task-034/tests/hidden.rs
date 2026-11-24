mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Full edition\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_034::TRACK_PROOF, 0x52540000 | 34, "See the submission step in REVIEW.md");
}
