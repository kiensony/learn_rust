use task_2897::*;

#[test]
fn transfer_case() {
    assert_eq!(
        big_endian_registers_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
        vec![
            (774u32, 1usize),
            (2316u32, 1usize),
            (5391u32, 1usize),
            (26u32, 1usize),
            (2079u32, 1usize),
            (7944u32, 1usize),
            (6656u32, 1usize),
            (3861u32, 1usize),
            (3081u32, 1usize),
            (1539u32, 1usize)
        ]
    );
}
