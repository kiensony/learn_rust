use task_1997::*;

#[test]
fn transfer_case() {
    assert_eq!(
        atomic_bit_union_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (0u32, 1usize),
            (3u32, 1usize),
            (7u32, 1usize),
            (15u32, 2usize),
            (31u32, 15usize)
        ]
    );
}
