use task_2947::*;

#[test]
fn edge_cases() {
    assert_eq!(latency_improvement_run_ledger(&[4]), vec![]);
    assert_eq!(
        latency_improvement_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(7u32, 1usize), (0u32, 2usize)]
    );
    assert_eq!(
        latency_improvement_run_ledger(&[0, 0, 0]),
        vec![(0u32, 1usize)]
    );
    assert_eq!(
        latency_improvement_run_ledger(&[255, 1, 255, 2]),
        vec![(254u32, 1usize), (253u32, 1usize)]
    );
    assert_eq!(
        latency_improvement_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(0u32, 2usize), (6u32, 1usize), (0u32, 2usize)]
    );
    assert_eq!(
        latency_improvement_run_ledger(&[17, 17, 1, 0, 2]),
        vec![(0u32, 1usize), (1u32, 1usize)]
    );
    assert_eq!(
        latency_improvement_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![(0u32, 2usize), (6u32, 1usize), (0u32, 2usize)]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2947::TRACK_PROOF, 0x52540000 | 2947);
}
