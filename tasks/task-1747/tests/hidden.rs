use task_1747::*;

#[test]
fn edge_cases() {
    assert_eq!(saturating_macro_run_ledger(&[4]), vec![(8u32, 1usize)]);
    assert_eq!(
        saturating_macro_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![
            (13u32, 1usize),
            (6u32, 2usize),
            (12u32, 1usize),
            (4u32, 1usize),
            (9u32, 1usize)
        ]
    );
    assert_eq!(
        saturating_macro_run_ledger(&[0, 0, 0]),
        vec![(4u32, 3usize)]
    );
    assert_eq!(
        saturating_macro_run_ledger(&[255, 1, 255, 2]),
        vec![
            (255u32, 1usize),
            (5u32, 1usize),
            (255u32, 1usize),
            (6u32, 1usize)
        ]
    );
    assert_eq!(
        saturating_macro_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (7u32, 1usize),
            (10u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (25u32, 1usize),
            (19u32, 1usize),
            (4u32, 1usize),
            (30u32, 1usize),
            (12u32, 1usize),
            (35u32, 1usize)
        ]
    );
    assert_eq!(
        saturating_macro_run_ledger(&[17, 17, 1, 0, 2]),
        vec![
            (21u32, 2usize),
            (5u32, 1usize),
            (4u32, 1usize),
            (6u32, 1usize)
        ]
    );
    assert_eq!(
        saturating_macro_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            (7u32, 1usize),
            (10u32, 1usize),
            (13u32, 1usize),
            (16u32, 1usize),
            (25u32, 1usize),
            (19u32, 1usize),
            (4u32, 1usize),
            (30u32, 1usize),
            (12u32, 1usize),
            (35u32, 1usize)
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1747::TRACK_PROOF, 0x52540000 | 1747);
}
