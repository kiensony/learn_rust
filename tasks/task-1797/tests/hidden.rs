use task_1797::*;

#[test]
fn edge_cases() {
    assert_eq!(generated_offset_run_ledger(&[4]), vec![(11u32, 1usize)]);
    assert_eq!(
        generated_offset_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![
            (16u32, 1usize),
            (9u32, 2usize),
            (15u32, 1usize),
            (7u32, 1usize),
            (12u32, 1usize)
        ]
    );
    assert_eq!(
        generated_offset_run_ledger(&[0, 0, 0]),
        vec![(7u32, 3usize)]
    );
    assert_eq!(
        generated_offset_run_ledger(&[255, 1, 255, 2]),
        vec![
            (262u32, 1usize),
            (8u32, 1usize),
            (262u32, 1usize),
            (9u32, 1usize)
        ]
    );
    assert_eq!(
        generated_offset_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (10u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (19u32, 1usize),
            (28u32, 1usize),
            (22u32, 1usize),
            (7u32, 1usize),
            (33u32, 1usize),
            (15u32, 1usize),
            (38u32, 1usize)
        ]
    );
    assert_eq!(
        generated_offset_run_ledger(&[17, 17, 1, 0, 2]),
        vec![
            (24u32, 2usize),
            (8u32, 1usize),
            (7u32, 1usize),
            (9u32, 1usize)
        ]
    );
    assert_eq!(
        generated_offset_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (10u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (19u32, 1usize),
            (28u32, 1usize),
            (22u32, 1usize),
            (7u32, 1usize),
            (33u32, 1usize),
            (15u32, 1usize),
            (38u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1797::TRACK_PROOF, 0x52540000 | 1797);
}
