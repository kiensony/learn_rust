mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "0.2.1\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_031::TRACK_PROOF, 0x52540000 | 31, "See the submission step in REVIEW.md");
}

#[test]
fn package_metadata() {
    assert_eq!(env!("CARGO_PKG_VERSION"), "0.2.1");
}
