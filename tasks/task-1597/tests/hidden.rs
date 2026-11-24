use task_1597::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_padding_run_ledger(&[4]),
        if cfg!(feature = "wide") {
            vec![(16u32, 1usize)]
        } else {
            vec![(4u32, 1usize)]
        }
    );
    assert_eq!(
        feature_padding_run_ledger(&[9, 2, 2, 8, 0, 5]),
        if cfg!(feature = "wide") {
            vec![
                (36u32, 1usize),
                (8u32, 2usize),
                (32u32, 1usize),
                (0u32, 1usize),
                (20u32, 1usize),
            ]
        } else {
            vec![
                (9u32, 1usize),
                (2u32, 2usize),
                (8u32, 1usize),
                (0u32, 1usize),
                (5u32, 1usize),
            ]
        }
    );
    assert_eq!(feature_padding_run_ledger(&[0, 0, 0]), vec![(0u32, 3usize)]);
    assert_eq!(
        feature_padding_run_ledger(&[255, 1, 255, 2]),
        if cfg!(feature = "wide") {
            vec![
                (1020u32, 1usize),
                (4u32, 1usize),
                (1020u32, 1usize),
                (8u32, 1usize),
            ]
        } else {
            vec![
                (255u32, 1usize),
                (1u32, 1usize),
                (255u32, 1usize),
                (2u32, 1usize),
            ]
        }
    );
    assert_eq!(
        feature_padding_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![
                (12u32, 1usize),
                (24u32, 1usize),
                (36u32, 1usize),
                (48u32, 1usize),
                (84u32, 1usize),
                (60u32, 1usize),
                (0u32, 1usize),
                (104u32, 1usize),
                (32u32, 1usize),
                (124u32, 1usize),
            ]
        } else {
            vec![
                (3u32, 1usize),
                (6u32, 1usize),
                (9u32, 1usize),
                (12u32, 1usize),
                (21u32, 1usize),
                (15u32, 1usize),
                (0u32, 1usize),
                (26u32, 1usize),
                (8u32, 1usize),
                (31u32, 1usize),
            ]
        }
    );
    assert_eq!(
        feature_padding_run_ledger(&[17, 17, 1, 0, 2]),
        if cfg!(feature = "wide") {
            vec![
                (68u32, 2usize),
                (4u32, 1usize),
                (0u32, 1usize),
                (8u32, 1usize),
            ]
        } else {
            vec![
                (17u32, 2usize),
                (1u32, 1usize),
                (0u32, 1usize),
                (2u32, 1usize),
            ]
        }
    );
    assert_eq!(
        feature_padding_run_ledger(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        if cfg!(feature = "wide") {
            vec![
                (12u32, 1usize),
                (24u32, 1usize),
                (36u32, 1usize),
                (48u32, 1usize),
                (84u32, 1usize),
                (60u32, 1usize),
                (0u32, 1usize),
                (104u32, 1usize),
                (32u32, 1usize),
                (124u32, 1usize),
            ]
        } else {
            vec![
                (3u32, 1usize),
                (6u32, 1usize),
                (9u32, 1usize),
                (12u32, 1usize),
                (21u32, 1usize),
                (15u32, 1usize),
                (0u32, 1usize),
                (26u32, 1usize),
                (8u32, 1usize),
                (31u32, 1usize),
            ]
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1597::TRACK_PROOF, 0x52540000 | 1597);
}
