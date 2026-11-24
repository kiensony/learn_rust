#[test]
fn empty_dequeue_then_repeated_arrivals() {
    assert_eq!(
        task_2984::service_wait_peaks(&[0, 9, 9, 9, 9, 0, 0, 9, 0]),
        vec![4u32, 5u32]
    );
}
