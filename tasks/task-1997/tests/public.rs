use task_1997::*;

#[test]
fn public_examples() {
    assert_eq!(
        atomic_bit_union_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (0u32, 1usize),
            (3u32, 1usize),
            (7u32, 1usize),
            (15u32, 2usize),
            (31u32, 5usize)
        ]
    );
    assert_eq!(atomic_bit_union_run_ledger(&[]), vec![]);
}
