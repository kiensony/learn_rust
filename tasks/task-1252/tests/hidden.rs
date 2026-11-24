use task_1252::*;

#[test]
fn edge_cases() {
    assert_eq!(command_arguments_total("-q -q -- -v"), 8);
    assert_eq!(command_arguments_total("--level=3 data --level=9"), 22);
    assert_eq!(command_arguments_total("a --bad --level=x b"), 16);
    assert_eq!(command_arguments_total("input -v -q input --level=255"), 25);
    assert_eq!(command_arguments_total("-- --level=4 x"), 12);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1252::TRACK_PROOF, 0x52540000 | 1252);
}
