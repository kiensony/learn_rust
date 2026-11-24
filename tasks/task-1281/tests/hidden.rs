use task_1281::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_trace("-q -q -- -v"), vec![2u32]);
    assert_eq!(option_terminator_trace("--level=3 data --level=9"), vec![]);
    assert_eq!(option_terminator_trace("a --bad --level=x b"), vec![]);
    assert_eq!(
        option_terminator_trace("input -v -q input --level=255"),
        vec![]
    );
    assert_eq!(option_terminator_trace("-- --level=4 x"), vec![9u32, 1u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1281::TRACK_PROOF, 0x52540000 | 1281);
}
