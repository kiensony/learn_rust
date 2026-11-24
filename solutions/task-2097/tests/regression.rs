use task_2097::*;

#[test]
fn transfer_case() {
    assert_eq!(
        double_buffer_receiver_run_ledger(&[9, 2, 2, 8, 0, 5, 5, 0, 8, 2, 2, 9]),
        vec![
            (9u32, 1usize),
            (2u32, 1usize),
            (5u32, 1usize),
            (8u32, 1usize)
        ]
    );
}
