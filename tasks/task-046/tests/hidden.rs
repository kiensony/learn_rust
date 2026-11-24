mod common;

#[test]
fn boundaries() {
    common::check(&[], "\n", "");
    common::check(&[""], "\n", "");
    common::check(&["½"], "½\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_046::TRACK_PROOF, 0x52540000 | 46, "See the submission step in REVIEW.md");
}
