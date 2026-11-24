use task_1277::*;

#[test]
fn edge_cases() {
    assert_eq!(explicit_levels_distinct("-q -q -- -v"), vec![]);
    assert_eq!(
        explicit_levels_distinct("--level=3 data --level=9"),
        vec![3u32, 9u32]
    );
    assert_eq!(explicit_levels_distinct("a --bad --level=x b"), vec![]);
    assert_eq!(
        explicit_levels_distinct("input -v -q input --level=255"),
        vec![255u32]
    );
    assert_eq!(explicit_levels_distinct("-- --level=4 x"), vec![4u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1277::TRACK_PROOF, 0x52540000 | 1277);
}
