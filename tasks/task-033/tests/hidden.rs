mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Gates open at nine.\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_033::TRACK_PROOF, 0x52540000 | 33, "See the submission step in REVIEW.md");
}
