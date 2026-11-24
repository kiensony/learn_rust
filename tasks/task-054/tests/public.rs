mod common;

#[test]
fn examples() {
    common::check(&[], "Select a drawer.\n", "");
    common::check(&["unused"], "Select a drawer.\n", "");
}

#[test]
fn shared_resource() {
    assert_eq!(include_str!("../prompt.txt"), "Select a drawer.\n");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_again")).output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"Select a drawer.\n");
}
