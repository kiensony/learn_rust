#[test]
fn empty_dequeue_then_repeated_arrivals() {
    assert_eq!(
        task_2994::drain_order_peaks(&[0, 9, 9, 9, 9, 0, 0, 9, 0]),
        vec![9u32]
    );
}
