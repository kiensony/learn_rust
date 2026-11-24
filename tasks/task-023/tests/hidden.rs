mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "42\n", "using fallback\n");
}

#[test]
fn submission_marker() {
    assert_eq!(task_023::TRACK_PROOF, 0x52540000 | 23, "See the submission step in REVIEW.md");
}
