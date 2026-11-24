mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "name\tscore\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_041::TRACK_PROOF, 0x52540000 | 41, "See the submission step in REVIEW.md");
}
