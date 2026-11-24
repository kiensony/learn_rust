mod common;

#[test]
fn boundaries() {
    common::check(&[""], "\n", "receipt copied\n");
    common::check(&["é"], "é\n", "receipt copied\n");
    common::check(&[], "\n", "receipt copied\n");
}

#[test]
fn submission_marker() {
    assert_eq!(task_057::TRACK_PROOF, 0x52540000 | 57, "See the submission step in REVIEW.md");
}
