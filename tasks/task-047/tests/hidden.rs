mod common;

#[test]
fn boundaries() {
    common::check(&[""], "===\n\n===\n", "");
    common::check(&["a b"], "===\na b\n===\n", "");
    common::check(&["雪"], "===\n雪\n===\n", "");
}

#[test]
fn reusable_value() {
    assert_eq!(task_047::divider(), "===");
}

#[test]
fn submission_marker() {
    assert_eq!(task_047::TRACK_PROOF, 0x52540000 | 47, "See the submission step in REVIEW.md");
}
