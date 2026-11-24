mod common;

#[test]
fn examples() {
    common::check(&[], "Full edition\n", "");
    common::check(&["unused"], "Full edition\n", "");
}

#[test]
fn selected_target() {
    let output = std::process::Command::new(env!("CARGO"))
        .args(["run", "--offline", "--quiet", "--example", "preview"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "Preview only\n");
}
