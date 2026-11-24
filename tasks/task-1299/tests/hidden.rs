use task_1299::*;

#[test]
fn edge_cases() {
    assert_eq!(option_terminator_frequent_readings("only", 0), vec![]);
    assert_eq!(
        option_terminator_frequent_readings("-q -q -- -v", 1),
        vec![2u32]
    );
    assert_eq!(
        option_terminator_frequent_readings("--level=3 data --level=9", 2),
        vec![]
    );
    assert_eq!(
        option_terminator_frequent_readings("a --bad --level=x b", 3),
        vec![]
    );
    assert_eq!(
        option_terminator_frequent_readings("input -v -q input --level=255", 256),
        vec![]
    );
    assert_eq!(
        option_terminator_frequent_readings("-- --level=4 x", 0),
        vec![]
    );
    assert_eq!(
        option_terminator_frequent_readings("-- --level=4 x", 1),
        vec![9u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1299::TRACK_PROOF, 0x52540000 | 1299);
}
