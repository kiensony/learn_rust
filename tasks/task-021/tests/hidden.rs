mod common;

#[test]
fn boundaries() {
    common::check(&[], " / \n", "");
    common::check(&["Town"], "Town / \n", "");
    common::check(&["Old Mill", "明日"], "Old Mill / 明日\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_021::TRACK_PROOF, 0x52540000 | 21, "See the submission step in REVIEW.md");
}
