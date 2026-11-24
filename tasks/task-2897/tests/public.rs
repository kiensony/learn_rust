use task_2897::*;

#[test]
fn public_examples() {
    assert_eq!(
        big_endian_registers_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (774u32, 1usize),
            (2316u32, 1usize),
            (5391u32, 1usize),
            (26u32, 1usize),
            (2079u32, 1usize)
        ]
    );
    assert_eq!(big_endian_registers_run_ledger(&[]), vec![]);
}
