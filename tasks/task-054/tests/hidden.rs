mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Select a drawer.\n", "");
}

#[test]
fn shared_resource() {
    assert_eq!(include_str!("../prompt.txt"), "Select a drawer.\n");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_again")).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"Select a drawer.\n");
}

#[test]
fn submission_marker() {
    assert_eq!(task_054::TRACK_PROOF, 0x52540000 | 54, "See the submission step in REVIEW.md");
}
