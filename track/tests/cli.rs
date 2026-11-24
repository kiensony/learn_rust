use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_track"))
        .args(args)
        .output()
        .expect("run track")
}

fn success(args: &[&str]) -> String {
    let output = cli(args);
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn the_beginner_entry_point_is_position_one() {
    let listing = success(&["list"]);
    assert!(listing.lines().next().unwrap().contains("task-011"));
    assert!(success(&["show", "11"]).contains("cargo run"));
    assert!(success(&["next", "task-011"]).contains("task-012"));
    assert!(success(&["hint", "011", "2"]).contains("src/main.rs"));
}

#[test]
fn search_is_case_insensitive_and_accepts_multiple_filters() {
    let listing = success(&["list", "SETUP", "--kind", "debugging"]);
    assert!(listing.contains("task-019"));
    assert!(!listing.contains("task-011"));
}

#[test]
fn legacy_ids_are_stable_and_jump_in_learning_order() {
    let task = success(&["show", "task-001"]);
    assert!(task.contains("0256"));
    assert!(task.contains("cargo test -p task_001"));
    assert!(success(&["next", "task-001"]).contains("0257"));
}

#[test]
fn invalid_inputs_produce_errors() {
    for args in [
        &["show", "../Cargo.toml"][..],
        &["show", "+11"],
        &["hint", "task-011", "4"],
        &["list", "--kind"],
        &["list", "--unknown"],
        &["next", "task-011", "task-012"],
    ] {
        let output = cli(args);
        assert!(!output.status.success(), "{args:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains("track:"));
    }
}
