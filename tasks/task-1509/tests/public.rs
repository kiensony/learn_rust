use task_1509::elapsed_seconds;

#[test]
fn two_ticks_take_half_a_minute() {
    assert_eq!(elapsed_seconds(2), Some(30));
}

#[test]
fn ten_ticks_take_two_and_a_half_minutes() {
    assert_eq!(elapsed_seconds(10), Some(150));
}
