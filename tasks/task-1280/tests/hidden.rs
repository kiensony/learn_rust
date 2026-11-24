use task_1280::*;

#[test]
fn edge_cases() {
    assert_eq!(explicit_levels_reverse("-q -q -- -v"), vec![]);
    assert_eq!(
        explicit_levels_reverse("--level=3 data --level=9"),
        vec![9u32, 3u32]
    );
    assert_eq!(explicit_levels_reverse("a --bad --level=x b"), vec![]);
    assert_eq!(
        explicit_levels_reverse("input -v -q input --level=255"),
        vec![255u32]
    );
    assert_eq!(explicit_levels_reverse("-- --level=4 x"), vec![4u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1280::TRACK_PROOF, 0x52540000 | 1280);
}
