use task_2748::*;

#[test]
fn public_examples() {
    assert_eq!(
        cli_path_depth_rolling_load("a a/z.rs a/b.rs .cargo/config", 1),
        vec![1u64, 2u64, 2u64, 2u64]
    );
    assert_eq!(cli_path_depth_rolling_load("", 0), vec![]);
}
