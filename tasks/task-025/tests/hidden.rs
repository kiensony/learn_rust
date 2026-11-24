mod common;

#[test]
fn boundaries() {
    common::check(&[], "[] \n", "");
    common::check(&["a b"], "[a b] a b\n", "");
    common::check(&["€"], "[€] €\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_025::TRACK_PROOF, 0x52540000 | 25, "See the submission step in REVIEW.md");
}
