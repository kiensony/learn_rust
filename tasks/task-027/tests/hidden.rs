mod common;

#[test]
fn boundaries() {
    common::check(&[], "\n", "");
    common::check(&["{}"], "{}\n", "");
    common::check(&["雪"], "雪\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_027::TRACK_PROOF, 0x52540000 | 27, "See the submission step in REVIEW.md");
}
