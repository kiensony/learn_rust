use task_1298::*;

#[test]
fn edge_cases() {
    assert_eq!(explicit_levels_rolling_load("only", 1), vec![]);
    assert_eq!(explicit_levels_rolling_load("-q -q -- -v", 2), vec![]);
    assert_eq!(
        explicit_levels_rolling_load("--level=3 data --level=9", 0),
        vec![]
    );
    assert_eq!(
        explicit_levels_rolling_load("a --bad --level=x b", 100),
        vec![]
    );
    assert_eq!(
        explicit_levels_rolling_load("input -v -q input --level=255", 3),
        vec![]
    );
    assert_eq!(
        explicit_levels_rolling_load("-- --level=4 x", 1),
        vec![4u64]
    );
    assert_eq!(
        explicit_levels_rolling_load("--level=3 data --level=9", 2),
        vec![12u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1298::TRACK_PROOF, 0x52540000 | 1298);
}
