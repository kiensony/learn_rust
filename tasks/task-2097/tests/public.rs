use task_2097::*;

#[test]
fn public_examples() {
    assert_eq!(
        double_buffer_receiver_run_ledger(&[9, 2, 2, 8, 0, 5]),
        vec![(9u32, 1usize), (2u32, 1usize), (5u32, 1usize)]
    );
    assert_eq!(double_buffer_receiver_run_ledger(&[]), vec![]);
}
