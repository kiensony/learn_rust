mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Dock three\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_053::TRACK_PROOF, 0x52540000 | 53, "See the submission step in REVIEW.md");
}
