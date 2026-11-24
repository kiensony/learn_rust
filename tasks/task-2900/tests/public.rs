use task_2900::*;

#[test]
fn public_examples() {
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[1539, 3081, 3861, 6656]
        ),
        Some(4)
    );
    assert_eq!(signed_sensor_magnitude_replay_mismatch(&[], &[]), None);
}
