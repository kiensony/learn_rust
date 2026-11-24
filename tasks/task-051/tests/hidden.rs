mod common;

#[test]
fn boundaries() {
    common::check(&[], "Arriving: \n", "");
    common::check(&["{X}"], "Arriving: {X}\n", "");
    common::check(&["ñ"], "Arriving: ñ\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_051::TRACK_PROOF, 0x52540000 | 51, "See the submission step in REVIEW.md");
}
