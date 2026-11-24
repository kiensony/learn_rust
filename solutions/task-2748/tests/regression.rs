use task_2748::*;

#[test]
fn transfer_case() {
    assert_eq!(
        cli_path_depth_rolling_load(
            "a a/z.rs a/b.rs .cargo/config a a/z.rs a/b.rs .cargo/config",
            3
        ),
        vec![5u64, 6u64, 5u64, 5u64, 5u64, 6u64]
    );
}
