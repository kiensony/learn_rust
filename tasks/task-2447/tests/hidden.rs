use task_2447::*;

#[test]
fn edge_cases() {
    assert_eq!(word_alignment_run_ledger(&[4]), vec![(4u32, 1usize)]);
    assert_eq!(
        word_alignment_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![
            (12u32, 1usize),
            (4u32, 2usize),
            (8u32, 1usize),
            (0u32, 1usize),
            (8u32, 1usize)
        ]
    );
    assert_eq!(word_alignment_run_ledger(&[0, 0, 0]), vec![(0u32, 3usize)]);
    assert_eq!(
        word_alignment_run_ledger(&[255, 1, 255, 2]),
        vec![
            (256u32, 1usize),
            (4u32, 1usize),
            (256u32, 1usize),
            (4u32, 1usize)
        ]
    );
    assert_eq!(
        word_alignment_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (4u32, 1usize),
            (8u32, 1usize),
            (12u32, 2usize),
            (24u32, 1usize),
            (16u32, 1usize),
            (0u32, 1usize),
            (28u32, 1usize),
            (8u32, 1usize),
            (32u32, 1usize)
        ]
    );
    assert_eq!(
        word_alignment_run_ledger(&[17, 17, 1, 0, 2]),
        vec![
            (20u32, 2usize),
            (4u32, 1usize),
            (0u32, 1usize),
            (4u32, 1usize)
        ]
    );
    assert_eq!(
        word_alignment_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (4u32, 1usize),
            (8u32, 1usize),
            (12u32, 2usize),
            (24u32, 1usize),
            (16u32, 1usize),
            (0u32, 1usize),
            (28u32, 1usize),
            (8u32, 1usize),
            (32u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2447::TRACK_PROOF, 0x52540000 | 2447);
}
