use task_2347::*;

#[test]
fn public_examples() {
    assert_eq!(
        paired_speedup_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(0u32, 2usize), (6u32, 1usize), (0u32, 2usize)]
    );
    assert_eq!(paired_speedup_run_ledger(&[]), vec![]);
}
