mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Platform 4\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_020::TRACK_PROOF, 0x52540000 | 20, "See the submission step in REVIEW.md");
}
