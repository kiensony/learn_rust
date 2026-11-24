mod common;

#[test]
fn examples() {
    common::check(&[], "Customer queue\n", "");
    common::check(&["unused"], "Customer queue\n", "");
}

#[test]
fn selected_target() {
    let output = std::process::Command::new(env!("CARGO"))
        .args(["run", "--offline", "--quiet"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Customer queue\n");
}
