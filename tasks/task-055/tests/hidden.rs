mod common;

#[test]
fn boundaries() {
    common::check(&[], "R-\n", "");
    common::check(&[" 2"], "R- 2\n", "");
    common::check(&["0"], "R-0\n", "");
}

#[test]
fn reusable_label() {
    assert_eq!(task_055::receipt(""), "R-");
    assert_eq!(task_055::receipt(" 2"), "R- 2");
    assert_eq!(task_055::receipt("0"), "R-0");
}

#[test]
fn submission_marker() {
    assert_eq!(task_055::TRACK_PROOF, 0x52540000 | 55, "See the submission step in REVIEW.md");
}
