use task_1298::*;

#[test]
fn public_examples() {
    assert_eq!(
        explicit_levels_rolling_load("--level=3 data --level=9", 1),
        vec![3u64, 9u64]
    );
    assert_eq!(explicit_levels_rolling_load("", 0), vec![]);
}
