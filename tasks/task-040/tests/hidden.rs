mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "desk ready\nline ready\n", "");
}

#[test]
fn reusable_value() {
    assert_eq!(task_040::STATUS, "ready");
}

#[test]
fn submission_marker() {
    assert_eq!(task_040::TRACK_PROOF, 0x52540000 | 40, "See the submission step in REVIEW.md");
}
