mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Dear Jo,\n\nSee you soon.\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_014::TRACK_PROOF, 0x52540000 | 14, "See the submission step in REVIEW.md");
}
