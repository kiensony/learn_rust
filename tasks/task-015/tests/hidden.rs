mod common;

#[test]
fn boundaries() {
    common::check(&[], "Ticket: \n", "");
    common::check(&["one two"], "Ticket: one two\n", "");
    common::check(&["雪"], "Ticket: 雪\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_015::TRACK_PROOF, 0x52540000 | 15, "See the submission step in REVIEW.md");
}
