mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "North gate\nOpen\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_012::TRACK_PROOF, 0x52540000 | 12, "See the submission step in REVIEW.md");
}
