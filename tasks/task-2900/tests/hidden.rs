use task_2900::*;

#[test]
fn edge_cases() {
    assert_eq!(signed_sensor_magnitude_replay_mismatch(&[4], &[]), None);
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[521, 2050, 1280, 777]),
        Some(3)
    );
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(&[0, 0, 0], &[]),
        Some(0)
    );
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[7944, 6656, 3861, 3081, 1539]
        ),
        Some(0)
    );
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(&[17, 17, 1, 0, 2], &[4369, 1]),
        None
    );
    assert_eq!(
        signed_sensor_magnitude_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[1539, 3081, 3861, 6656, 7944, 1]
        ),
        Some(5)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2900::TRACK_PROOF, 0x52540000 | 2900);
}
