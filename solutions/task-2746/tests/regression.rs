use task_2746::*;

#[test]
fn transfer_case() {
    assert_eq!(
        cli_basenames_threshold_crossing(
            "a a/z.rs a/b.rs .cargo/config a a/z.rs a/b.rs .cargo/config",
            1
        ),
        Some((0, 1))
    );
}
