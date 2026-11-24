#[test]
fn empty_dequeue_then_repeated_arrivals() {
    assert_eq!(
        task_2979::queue_depth_longest_run(&[0, 9, 9, 9, 9, 0, 0, 9, 0]),
        1
    );
}
