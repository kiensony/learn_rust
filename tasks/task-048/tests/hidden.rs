mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "License: MIT\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_048::TRACK_PROOF, 0x52540000 | 48, "See the submission step in REVIEW.md");
}

#[test]
fn package_metadata() {
    assert_eq!(env!("CARGO_PKG_LICENSE"), "MIT");
}
