mod common;

#[test]
fn boundaries() {
    common::check(&["", "中"], "Customer queue\n", "");
}

#[test]
fn operator_still_available() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_operator")).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"Operator desk\n");
}

#[test]
fn submission_marker() {
    assert_eq!(task_043::TRACK_PROOF, 0x52540000 | 43, "See the submission step in REVIEW.md");
}
