use task_2697::*;

#[test]
fn edge_cases() {
    assert_eq!(lending_first_byte_run_ledger(&[4]), vec![(4u32, 1usize)]);
    assert_eq!(
        lending_first_byte_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(9u32, 1usize), (8u32, 1usize)]
    );
    assert_eq!(
        lending_first_byte_run_ledger(&[0, 0, 0]),
        vec![(0u32, 1usize)]
    );
    assert_eq!(
        lending_first_byte_run_ledger(&[255, 1, 255, 2]),
        vec![(255u32, 1usize), (2u32, 1usize)]
    );
    assert_eq!(
        lending_first_byte_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (3u32, 1usize),
            (12u32, 1usize),
            (0u32, 1usize),
            (31u32, 1usize)
        ]
    );
    assert_eq!(
        lending_first_byte_run_ledger(&[17, 17, 1, 0, 2]),
        vec![(17u32, 1usize), (0u32, 1usize)]
    );
    assert_eq!(
        lending_first_byte_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (3u32, 1usize),
            (12u32, 1usize),
            (0u32, 1usize),
            (31u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2697::TRACK_PROOF, 0x52540000 | 2697);
}
