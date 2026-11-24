use task_2597::*;

#[test]
fn edge_cases() {
    assert_eq!(odd_element_proof_run_ledger(&[4]), vec![]);
    assert_eq!(
        odd_element_proof_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(3u32, 1usize), (9u32, 1usize), (6u32, 1usize)]
    );
    assert_eq!(
        odd_element_proof_run_ledger(&[0, 0, 0]),
        vec![(1u32, 1usize)]
    );
    assert_eq!(
        odd_element_proof_run_ledger(&[255, 1, 255, 2]),
        vec![(2u32, 1usize), (3u32, 1usize)]
    );
    assert_eq!(
        odd_element_proof_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (7u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (27u32, 1usize),
            (32u32, 1usize)
        ]
    );
    assert_eq!(
        odd_element_proof_run_ledger(&[17, 17, 1, 0, 2]),
        vec![(18u32, 1usize), (1u32, 1usize)]
    );
    assert_eq!(
        odd_element_proof_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (7u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (27u32, 1usize),
            (32u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2597::TRACK_PROOF, 0x52540000 | 2597);
}
