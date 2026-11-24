use task_2147::*;

#[test]
fn transfer_case() {
    assert_eq!(
        priority_removal_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (31u32, 2usize),
            (26u32, 2usize),
            (21u32, 2usize),
            (15u32, 2usize),
            (12u32, 2usize),
            (9u32, 2usize),
            (8u32, 2usize),
            (6u32, 2usize),
            (3u32, 2usize),
            (0u32, 2usize)
        ]
    );
}
