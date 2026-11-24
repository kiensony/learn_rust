#[test]
fn empty_dequeue_then_repeated_arrivals() {
    assert_eq!(
        task_2957::journal_replay_distinct(&[0, 9, 9, 9, 9, 0, 0, 9, 0]),
        vec![9u32]
    );
}
