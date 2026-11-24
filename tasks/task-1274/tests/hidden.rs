use task_1274::*;

#[test]
fn edge_cases() {
    assert_eq!(explicit_levels_peaks("-q -q -- -v"), vec![]);
    assert_eq!(
        explicit_levels_peaks("--level=3 data --level=9"),
        vec![3u32, 9u32]
    );
    assert_eq!(explicit_levels_peaks("a --bad --level=x b"), vec![]);
    assert_eq!(
        explicit_levels_peaks("input -v -q input --level=255"),
        vec![255u32]
    );
    assert_eq!(explicit_levels_peaks("-- --level=4 x"), vec![4u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1274::TRACK_PROOF, 0x52540000 | 1274);
}
