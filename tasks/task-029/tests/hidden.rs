mod common;

#[test]
fn boundaries() {
    common::check(&[], "\n", "");
    common::check(&[" sample "], " sample \n", "");
    common::check(&["{}"], "{}\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_029::TRACK_PROOF, 0x52540000 | 29, "See the submission step in REVIEW.md");
}
