use task_2197::*;

#[test]
fn public_examples() {
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (0u32, 5usize),
            (1u32, 1usize),
            (6u32, 1usize),
            (0u32, 1usize),
            (5u32, 1usize),
            (0u32, 1usize)
        ]
    );
    assert_eq!(sorted_insertion_moves_run_ledger(&[]), vec![]);
}
