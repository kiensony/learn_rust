use task_1697::*;

#[test]
fn transfer_case() {
    assert_eq!(
        minor_version_run_ledger("1.0.0 1.2.0 2.0.0 1.0.0 1.2.0 2.0.0"),
        vec![
            (0u32, 1usize),
            (2u32, 1usize),
            (0u32, 2usize),
            (2u32, 1usize),
            (0u32, 1usize)
        ]
    );
}
