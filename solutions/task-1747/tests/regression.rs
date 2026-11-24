use task_1747::*;

#[test]
fn transfer_case() {
    assert_eq!(
        saturating_macro_run_ledger(&[
            3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
        ]),
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
            (35u32, 2usize),
            (12u32, 1usize),
            (30u32, 1usize),
            (4u32, 1usize),
            (19u32, 1usize),
            (25u32, 1usize),
            (16u32, 1usize),
            (13u32, 1usize),
            (10u32, 1usize),
            (7u32, 1usize)
        ]
    );
}
