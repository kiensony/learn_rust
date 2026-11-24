use task_2447::*;

#[test]
fn public_examples() {
    assert_eq!(
        word_alignment_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (4u32, 1usize),
            (8u32, 1usize),
            (12u32, 2usize),
            (24u32, 1usize),
            (16u32, 1usize),
            (0u32, 1usize),
            (28u32, 1usize),
            (8u32, 1usize),
            (32u32, 1usize)
        ]
    );
    assert_eq!(word_alignment_run_ledger(&[]), vec![]);
}
