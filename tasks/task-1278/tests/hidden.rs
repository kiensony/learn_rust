use task_1278::*;

#[test]
fn edge_cases() {
    assert_eq!(explicit_levels_steps("-q -q -- -v"), vec![]);
    assert_eq!(
        explicit_levels_steps("--level=3 data --level=9"),
        vec![6u32]
    );
    assert_eq!(explicit_levels_steps("a --bad --level=x b"), vec![]);
    assert_eq!(
        explicit_levels_steps("input -v -q input --level=255"),
        vec![]
    );
    assert_eq!(explicit_levels_steps("-- --level=4 x"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1278::TRACK_PROOF, 0x52540000 | 1278);
}
