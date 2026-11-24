use task_1698::*;

#[test]
fn transfer_case() {
    assert_eq!(
        patch_version_rolling_load("12.4.8 2.9.3 12.4.8 12.4.8 2.9.3 12.4.8", 3),
        vec![19u64, 19u64, 19u64, 19u64]
    );
}
