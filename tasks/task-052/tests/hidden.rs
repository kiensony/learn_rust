mod common;

#[test]
fn boundaries() {
    common::check(&[""], "\n", "");
    common::check(&["{}"], "{}\n", "");
    common::check(&["a", "b"], "a\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_052::TRACK_PROOF, 0x52540000 | 52, "See the submission step in REVIEW.md");
}
