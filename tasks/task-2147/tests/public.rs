use task_2147::*;

#[test]
fn public_examples() {
    assert_eq!(
        priority_removal_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (31u32, 1usize),
            (26u32, 1usize),
            (21u32, 1usize),
            (15u32, 1usize),
            (12u32, 1usize),
            (9u32, 1usize),
            (8u32, 1usize),
            (6u32, 1usize),
            (3u32, 1usize),
            (0u32, 1usize)
        ]
    );
    assert_eq!(priority_removal_run_ledger(&[]), vec![]);
}
