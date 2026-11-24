mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Parcel accepted.\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_028::TRACK_PROOF, 0x52540000 | 28, "See the submission step in REVIEW.md");
}
