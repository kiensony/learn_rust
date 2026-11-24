mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Bag #7\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_013::TRACK_PROOF, 0x52540000 | 13, "See the submission step in REVIEW.md");
}
