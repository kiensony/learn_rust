use task_2947::*;

#[test]
fn transfer_case() {
    assert_eq!(
        latency_improvement_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (0u32, 2usize),
            (6u32, 1usize),
            (0u32, 2usize),
            (23u32, 1usize),
            (26u32, 1usize),
            (0u32, 1usize),
            (3u32, 2usize)
        ]
    );
}
