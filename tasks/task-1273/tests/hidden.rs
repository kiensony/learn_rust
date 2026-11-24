use task_1273::*;

#[test]
fn edge_cases() {
    assert_eq!(explicit_levels_bounds("-q -q -- -v"), None);
    assert_eq!(
        explicit_levels_bounds("--level=3 data --level=9"),
        Some((3u32, 9u32))
    );
    assert_eq!(explicit_levels_bounds("a --bad --level=x b"), None);
    assert_eq!(
        explicit_levels_bounds("input -v -q input --level=255"),
        Some((255u32, 255u32))
    );
    assert_eq!(explicit_levels_bounds("-- --level=4 x"), Some((4u32, 4u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1273::TRACK_PROOF, 0x52540000 | 1273);
}
