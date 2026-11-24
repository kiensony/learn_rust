use task_1797::*;

#[test]
fn public_examples() {
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
    assert_eq!(generated_offset_run_ledger(&[]), vec![]);
}
