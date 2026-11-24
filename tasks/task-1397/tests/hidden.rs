use task_1397::*;

#[test]
fn edge_cases() {
    assert_eq!(frame_offsets_run_ledger(&[0]), vec![(0u32, 1usize)]);
    assert_eq!(
        frame_offsets_run_ledger(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![
            (0u32, 1usize),
            (3u32, 1usize),
            (5u32, 1usize),
            (6u32, 1usize)
        ]
    );
    assert_eq!(frame_offsets_run_ledger(&[3, 1]), vec![]);
    assert_eq!(
        frame_offsets_run_ledger(&[1, 0, 0, 1, 255]),
        vec![(0u32, 1usize), (2u32, 1usize), (3u32, 1usize)]
    );
    assert_eq!(
        frame_offsets_run_ledger(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![(0u32, 1usize), (3u32, 1usize), (6u32, 1usize)]
    );
    assert_eq!(frame_offsets_run_ledger(&[255, 0]), vec![]);
    assert_eq!(
        frame_offsets_run_ledger(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![
            (0u32, 1usize),
            (3u32, 1usize),
            (5u32, 1usize),
            (6u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1397::TRACK_PROOF, 0x52540000 | 1397);
}
