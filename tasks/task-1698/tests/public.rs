use task_1698::*;

#[test]
fn public_examples() {
    assert_eq!(
        patch_version_rolling_load("12.4.8 2.9.3 12.4.8", 1),
        vec![8u64, 3u64, 8u64]
    );
    assert_eq!(patch_version_rolling_load("", 0), vec![]);
}
