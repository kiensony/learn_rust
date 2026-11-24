use task_1289::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_longest_run("-q -q -- -v"), 1);
    assert_eq!(option_terminator_longest_run("--level=3 data --level=9"), 0);
    assert_eq!(option_terminator_longest_run("a --bad --level=x b"), 0);
    assert_eq!(
        option_terminator_longest_run("input -v -q input --level=255"),
        0
    );
    assert_eq!(option_terminator_longest_run("-- --level=4 x"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1289::TRACK_PROOF, 0x52540000 | 1289);
}
