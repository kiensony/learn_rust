use task_1287::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_distinct("-q -q -- -v"), vec![2u32]);
    assert_eq!(
        option_terminator_distinct("--level=3 data --level=9"),
        vec![]
    );
    assert_eq!(option_terminator_distinct("a --bad --level=x b"), vec![]);
    assert_eq!(
        option_terminator_distinct("input -v -q input --level=255"),
        vec![]
    );
    assert_eq!(
        option_terminator_distinct("-- --level=4 x"),
        vec![1u32, 9u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1287::TRACK_PROOF, 0x52540000 | 1287);
}
