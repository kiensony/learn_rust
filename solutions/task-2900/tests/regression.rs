use task_2900::*;

#[test]
fn transfer_case() {
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[1539, 3081, 3861, 6656, 7944, 2079, 26, 5391, 2316, 774]
        ),
        None
    );
}
