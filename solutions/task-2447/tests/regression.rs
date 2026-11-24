use task_2447::*;

#[test]
fn transfer_case() {
    assert_eq!(
        word_alignment_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (4u32, 1usize),
            (8u32, 1usize),
            (12u32, 2usize),
            (24u32, 1usize),
            (16u32, 1usize),
            (0u32, 1usize),
            (28u32, 1usize),
            (8u32, 1usize),
            (32u32, 2usize),
            (8u32, 1usize),
            (28u32, 1usize),
            (0u32, 1usize),
            (16u32, 1usize),
            (24u32, 1usize),
            (12u32, 2usize),
            (8u32, 1usize),
            (4u32, 1usize)
        ]
    );
}
