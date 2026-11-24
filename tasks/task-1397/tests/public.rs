use task_1397::*;

#[test]
fn public_examples() {
    assert_eq!(
        frame_offsets_run_ledger(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![
            (0u32, 1usize),
            (3u32, 1usize),
            (5u32, 1usize),
            (6u32, 1usize)
        ]
    );
    assert_eq!(frame_offsets_run_ledger(&[]), vec![]);
}
