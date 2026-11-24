mod common;

#[test]
fn boundaries() {
    common::check(&[""], "Item #\nPainted clay\n", "");
    common::check(&["A B"], "Item #A B\nPainted clay\n", "");
    common::check(&[], "Item #\nPainted clay\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_059::TRACK_PROOF, 0x52540000 | 59, "See the submission step in REVIEW.md");
}
