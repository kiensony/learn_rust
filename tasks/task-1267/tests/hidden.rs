use task_1267::*;

#[test]
fn edge_cases() {
    assert_eq!(positional_options_distinct("-q -q -- -v"), vec![]);
    assert_eq!(
        positional_options_distinct("--level=3 data --level=9"),
        vec![4u32]
    );
    assert_eq!(
        positional_options_distinct("a --bad --level=x b"),
        vec![1u32]
    );
    assert_eq!(
        positional_options_distinct("input -v -q input --level=255"),
        vec![5u32]
    );
    assert_eq!(positional_options_distinct("-- --level=4 x"), vec![1u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1267::TRACK_PROOF, 0x52540000 | 1267);
}
