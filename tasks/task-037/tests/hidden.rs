mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "PAID\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_037::TRACK_PROOF, 0x52540000 | 37, "See the submission step in REVIEW.md");
}
