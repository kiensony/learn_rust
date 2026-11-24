use task_2749::*;

#[test]
fn transfer_case() {
    assert_eq!(
        cli_rust_sources_frequent_readings(
            "a a/z.rs a/b.rs .cargo/config a a/z.rs a/b.rs .cargo/config",
            2
        ),
        vec![1u32, 2u32]
    );
}
