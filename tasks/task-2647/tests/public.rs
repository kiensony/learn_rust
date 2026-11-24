use task_2647::*;

#[test]
fn public_examples() {
    assert_eq!(
        embedded_flags_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (3u32, 1usize),
            (6u32, 1usize),
            (9u32, 1usize),
            (12u32, 1usize),
            (5u32, 1usize),
            (15u32, 1usize),
            (0u32, 1usize),
            (10u32, 1usize),
            (8u32, 1usize),
            (15u32, 1usize)
        ]
    );
    assert_eq!(embedded_flags_run_ledger(&[]), vec![]);
}
