use task_2749::*;

#[test]
fn public_examples() {
    assert_eq!(
        cli_rust_sources_frequent_readings("a a/z.rs a/b.rs .cargo/config", 3),
        vec![1u32, 2u32]
    );
    assert_eq!(cli_rust_sources_frequent_readings("", 0), vec![]);
}
