use task_1397::*;

#[test]
fn transfer_case() {
    assert_eq!(
        frame_offsets_run_ledger(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2, 2, 9, 9, 3, 0, 4, 1, 3, 7, 2]),
        vec![
            (0u32, 1usize),
            (3u32, 1usize),
            (5u32, 1usize),
            (6u32, 1usize),
            (10u32, 1usize),
            (13u32, 1usize)
        ]
    );
}
