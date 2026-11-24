use task_1997::*;

#[test]
fn edge_cases() {
    assert_eq!(atomic_bit_union_run_ledger(&[4]), vec![(0u32, 1usize)]);
    assert_eq!(
        atomic_bit_union_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(0u32, 1usize), (9u32, 1usize), (11u32, 4usize)]
    );
    assert_eq!(
        atomic_bit_union_run_ledger(&[0, 0, 0]),
        vec![(0u32, 3usize)]
    );
    assert_eq!(
        atomic_bit_union_run_ledger(&[255, 1, 255, 2]),
        vec![(0u32, 1usize), (255u32, 3usize)]
    );
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
    assert_eq!(
        atomic_bit_union_run_ledger(&[17, 17, 1, 0, 2]),
        vec![(0u32, 1usize), (17u32, 4usize)]
    );
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
}

#[test]
fn submission_marker() {
    assert_eq!(task_1997::TRACK_PROOF, 0x52540000 | 1997);
}
