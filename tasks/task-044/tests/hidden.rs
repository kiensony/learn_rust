mod common;

#[test]
fn boundaries() {
    common::check(&[], "Room: \n", "");
    common::check(&[""], "Room: \n", "");
    common::check(&["South 2"], "Room: South 2\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_044::TRACK_PROOF, 0x52540000 | 44, "See the submission step in REVIEW.md");
}
