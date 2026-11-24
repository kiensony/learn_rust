use task_1847::*;

#[test]
fn public_examples() {
    assert_eq!(
        shard_maxima_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (9u32, 1usize),
            (21u32, 1usize),
            (26u32, 1usize),
            (31u32, 1usize)
        ]
    );
    assert_eq!(shard_maxima_run_ledger(&[]), vec![]);
}
