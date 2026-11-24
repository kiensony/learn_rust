mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Counts seats on the ferry\n", "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_035::TRACK_PROOF, 0x52540000 | 35, "See the submission step in REVIEW.md");
}

#[test]
fn package_metadata() {
    assert_eq!(env!("CARGO_PKG_DESCRIPTION"), "Counts seats on the ferry");
}
