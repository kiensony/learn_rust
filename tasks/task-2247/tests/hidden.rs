use task_2247::*;

#[test]
fn edge_cases() {
    assert_eq!(borrowed_offsets_run_ledger("a"), vec![]);
    assert_eq!(
        borrowed_offsets_run_ledger("a\nblue\nblue\r\nz\n"),
        vec![
            (1u32, 1usize),
            (6u32, 1usize),
            (12u32, 1usize),
            (14u32, 1usize)
        ]
    );
    assert_eq!(
        borrowed_offsets_run_ledger("   x\té\n\ncat\n"),
        vec![(7u32, 1usize), (8u32, 1usize), (12u32, 1usize)]
    );
    assert_eq!(
        borrowed_offsets_run_ledger("nine\nq\nthree\nx\n"),
        vec![
            (4u32, 1usize),
            (6u32, 1usize),
            (12u32, 1usize),
            (14u32, 1usize)
        ]
    );
    assert_eq!(
        borrowed_offsets_run_ledger("a b c\n\r\nup\tdown"),
        vec![(5u32, 1usize), (7u32, 1usize)]
    );
    assert_eq!(
        borrowed_offsets_run_ledger("\n\nzz\n"),
        vec![(0u32, 1usize), (1u32, 1usize), (4u32, 1usize)]
    );
    assert_eq!(
        borrowed_offsets_run_ledger("a\nblue\nblue\r\nz\n"),
        vec![
            (1u32, 1usize),
            (6u32, 1usize),
            (12u32, 1usize),
            (14u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2247::TRACK_PROOF, 0x52540000 | 2247);
}
