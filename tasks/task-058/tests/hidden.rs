mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "task_058/1.3.0\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_058::TRACK_PROOF, 0x52540000 | 58, "See the submission step in REVIEW.md");
}

#[test]
fn package_metadata() {
    assert_eq!(env!("CARGO_PKG_VERSION"), "1.3.0");
}
