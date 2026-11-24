use task_1897::*;

#[test]
fn public_examples() {
    assert_eq!(
        weak_read_handles_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (3u32, 1usize),
            (2u32, 1usize),
            (1u32, 1usize),
            (0u32, 1usize),
            (1u32, 1usize),
            (3u32, 1usize),
            (0u32, 1usize),
            (2u32, 1usize),
            (0u32, 1usize),
            (3u32, 1usize)
        ]
    );
    assert_eq!(weak_read_handles_run_ledger(&[]), vec![]);
}
