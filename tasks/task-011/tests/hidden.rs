mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Hello, Rust!\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_011::TRACK_PROOF, 0x52540000 | 11, "See the submission step in REVIEW.md");
}
