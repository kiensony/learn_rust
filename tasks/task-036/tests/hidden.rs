mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Pier 8\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_036::TRACK_PROOF, 0x52540000 | 36, "See the submission step in REVIEW.md");
}
