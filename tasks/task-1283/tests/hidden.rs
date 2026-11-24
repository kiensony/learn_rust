use task_1283::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_bounds("-q -q -- -v"), Some((2u32, 2u32)));
    assert_eq!(option_terminator_bounds("--level=3 data --level=9"), None);
    assert_eq!(option_terminator_bounds("a --bad --level=x b"), None);
    assert_eq!(
        option_terminator_bounds("input -v -q input --level=255"),
        None
    );
    assert_eq!(
        option_terminator_bounds("-- --level=4 x"),
        Some((1u32, 9u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1283::TRACK_PROOF, 0x52540000 | 1283);
}
