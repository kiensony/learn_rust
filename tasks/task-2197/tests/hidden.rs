use task_2197::*;

#[test]
fn edge_cases() {
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[4]),
        vec![(0u32, 1usize)]
    );
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![
            (0u32, 1usize),
            (1u32, 3usize),
            (4u32, 1usize),
            (2u32, 1usize)
        ]
    );
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[0, 0, 0]),
        vec![(0u32, 3usize)]
    );
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[255, 1, 255, 2]),
        vec![
            (0u32, 1usize),
            (1u32, 1usize),
            (0u32, 1usize),
            (2u32, 1usize)
        ]
    );
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (0u32, 5usize),
            (1u32, 1usize),
            (6u32, 1usize),
            (0u32, 1usize),
            (5u32, 1usize),
            (0u32, 1usize)
        ]
    );
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[17, 17, 1, 0, 2]),
        vec![
            (0u32, 2usize),
            (2u32, 1usize),
            (3u32, 1usize),
            (2u32, 1usize)
        ]
    );
    assert_eq!(
        sorted_insertion_moves_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (0u32, 5usize),
            (1u32, 1usize),
            (6u32, 1usize),
            (0u32, 1usize),
            (5u32, 1usize),
            (0u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2197::TRACK_PROOF, 0x52540000 | 2197);
}
