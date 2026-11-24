mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Beacon on\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_026::TRACK_PROOF, 0x52540000 | 26, "See the submission step in REVIEW.md");
}
