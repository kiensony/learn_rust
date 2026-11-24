use task_1259::*;

#[test]
fn edge_cases() {
    assert_eq!(command_arguments_longest_run("-q -q -- -v"), 4);
    assert_eq!(command_arguments_longest_run("--level=3 data --level=9"), 1);
    assert_eq!(command_arguments_longest_run("a --bad --level=x b"), 1);
    assert_eq!(
        command_arguments_longest_run("input -v -q input --level=255"),
        2
    );
    assert_eq!(command_arguments_longest_run("-- --level=4 x"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1259::TRACK_PROOF, 0x52540000 | 1259);
}
