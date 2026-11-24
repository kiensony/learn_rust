use task_2547::*;

#[test]
fn transfer_case() {
    assert_eq!(
        abi_flags_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (1u32, 6usize),
            (0u32, 1usize),
            (1u32, 6usize),
            (0u32, 1usize),
            (1u32, 6usize)
        ]
    );
}
