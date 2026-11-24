use task_1256::*;

#[test]
fn edge_cases() {
    assert_eq!(command_arguments_descending_steps("-q -q -- -v"), 0);
    assert_eq!(
        command_arguments_descending_steps("--level=3 data --level=9"),
        1
    );
    assert_eq!(command_arguments_descending_steps("a --bad --level=x b"), 1);
    assert_eq!(
        command_arguments_descending_steps("input -v -q input --level=255"),
        1
    );
    assert_eq!(command_arguments_descending_steps("-- --level=4 x"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1256::TRACK_PROOF, 0x52540000 | 1256);
}
