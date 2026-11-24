use task_1847::*;

#[test]
fn transfer_case() {
    assert_eq!(
        shard_maxima_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (9u32, 1usize),
            (21u32, 1usize),
            (26u32, 1usize),
            (31u32, 1usize),
            (26u32, 1usize),
            (21u32, 1usize),
            (6u32, 1usize)
        ]
    );
}
