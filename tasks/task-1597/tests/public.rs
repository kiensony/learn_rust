use task_1597::*;

#[test]
fn public_examples() {
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
    assert_eq!(feature_padding_run_ledger(&[]), vec![]);
}
