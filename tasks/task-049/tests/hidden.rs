mod common;

#[test]
fn boundaries() {
    common::check(&[], "arguments=0\n", "");
    common::check(&[""], "arguments=1\n", "");
    common::check(&["a", "b", "c"], "arguments=3\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_049::TRACK_PROOF, 0x52540000 | 49, "See the submission step in REVIEW.md");
}
