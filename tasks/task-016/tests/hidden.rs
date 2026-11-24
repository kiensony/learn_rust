mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "24 seeds\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_016::TRACK_PROOF, 0x52540000 | 16, "See the submission step in REVIEW.md");
}
