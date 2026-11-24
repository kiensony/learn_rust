use task_2197::*;

#[test]
fn transfer_case() {
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (0u32, 5usize),
            (1u32, 1usize),
            (6u32, 1usize),
            (0u32, 1usize),
            (5u32, 1usize),
            (0u32, 2usize),
            (7u32, 1usize),
            (2u32, 1usize),
            (12u32, 1usize),
            (5u32, 1usize),
            (4u32, 1usize),
            (8u32, 1usize),
            (10u32, 1usize),
            (14u32, 1usize),
            (16u32, 1usize)
        ]
    );
}
