use task_1509::elapsed_seconds;

#[test]
fn zero_and_overflow_boundaries() {
    assert_eq!(elapsed_seconds(0), Some(0));
    let largest = u64::MAX / 15;
    assert_eq!(elapsed_seconds(largest), Some(largest * 15));
    assert_eq!(elapsed_seconds(largest + 1), None);
}
