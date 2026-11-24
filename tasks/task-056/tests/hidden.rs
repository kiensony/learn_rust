mod common;

#[test]
fn boundaries() {
    common::check(&["", "Fri"], "TRAIL CARD\n[]\n(Fri)\n", "");
    common::check(&["雪", ""], "TRAIL CARD\n[雪]\n()\n", "");
    common::check(&[], "TRAIL CARD\n[]\n()\n", "");
    common::check(&["A", "D", "extra"], "TRAIL CARD\n[A]\n(D)\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_056::TRACK_PROOF, 0x52540000 | 56, "See the submission step in REVIEW.md");
}
