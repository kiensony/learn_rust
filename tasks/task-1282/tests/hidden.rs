use task_1282::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_total("-q -q -- -v"), 2);
    assert_eq!(option_terminator_total("--level=3 data --level=9"), 0);
    assert_eq!(option_terminator_total("a --bad --level=x b"), 0);
    assert_eq!(option_terminator_total("input -v -q input --level=255"), 0);
    assert_eq!(option_terminator_total("-- --level=4 x"), 10);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1282::TRACK_PROOF, 0x52540000 | 1282);
}
