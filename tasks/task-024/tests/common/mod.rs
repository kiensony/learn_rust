use std::process::Command;

pub fn check(args: &[&str], expected_stdout: &str, expected_stderr: &str) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_task_024"));
    command.args(args);
    let output = command.output().expect("launch the exercise binary");
    assert!(output.status.success(), "program failed: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected_stdout, "standard output");
    assert_eq!(String::from_utf8(output.stderr).unwrap(), expected_stderr, "standard error");
}
