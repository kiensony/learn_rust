use task_1447::*;

#[test]
fn edge_cases() {
    assert_eq!(elapsed_seconds_run_ledger(&[4]), vec![(0u32, 1usize)]);
    assert_eq!(
        elapsed_seconds_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(0u32, 6usize)]
    );
    assert_eq!(elapsed_seconds_run_ledger(&[0, 0, 0]), vec![(0u32, 3usize)]);
    assert_eq!(
        elapsed_seconds_run_ledger(&[255, 1, 255, 2]),
        vec![
            (4u32, 1usize),
            (0u32, 1usize),
            (4u32, 1usize),
            (0u32, 1usize)
        ]
    );
    assert_eq!(
        elapsed_seconds_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(0u32, 10usize)]
    );
    assert_eq!(
        elapsed_seconds_run_ledger(&[17, 17, 1, 0, 2]),
        vec![(0u32, 5usize)]
    );
    assert_eq!(
        elapsed_seconds_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(0u32, 10usize)]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1447::TRACK_PROOF, 0x52540000 | 1447);
}
