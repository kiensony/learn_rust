mod common;

#[test]
fn boundaries() {
    common::check(&[], " <- \n", "");
    common::check(&["lights"], " <- lights\n", "");
    common::check(&["run", ""], " <- run\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_060::TRACK_PROOF, 0x52540000 | 60, "See the submission step in REVIEW.md");
}
