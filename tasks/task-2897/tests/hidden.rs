use task_2897::*;

#[test]
fn edge_cases() {
    assert_eq!(big_endian_registers_run_ledger(&[4]), vec![]);
    assert_eq!(
        big_endian_registers_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(2306u32, 1usize), (520u32, 1usize), (5u32, 1usize)]
    );
    assert_eq!(
        big_endian_registers_run_ledger(&[0, 0, 0]),
        vec![(0u32, 1usize)]
    );
    assert_eq!(
        big_endian_registers_run_ledger(&[255, 1, 255, 2]),
        vec![(65281u32, 1usize), (65282u32, 1usize)]
    );
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
    assert_eq!(
        big_endian_registers_run_ledger(&[17, 17, 1, 0, 2]),
        vec![(4369u32, 1usize), (256u32, 1usize)]
    );
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
}

#[test]
fn submission_marker() {
    assert_eq!(task_2897::TRACK_PROOF, 0x52540000 | 2897);
}
