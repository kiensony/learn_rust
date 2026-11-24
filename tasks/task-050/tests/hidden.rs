mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Trip log\nUsage: trip <place>\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_050::TRACK_PROOF, 0x52540000 | 50, "See the submission step in REVIEW.md");
}
