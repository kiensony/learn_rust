use task_2891::*;

#[test]
fn edge_cases() {
    assert_eq!(
        signed_sensor_magnitude_trace(&[9, 2, 2, 8, 0, 5]),
        vec![521u32, 2050u32, 1280u32]
    );
    assert_eq!(signed_sensor_magnitude_trace(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        signed_sensor_magnitude_trace(&[255, 1, 255, 2]),
        vec![511u32, 767u32]
    );
    assert_eq!(
        signed_sensor_magnitude_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1539u32, 3081u32, 3861u32, 6656u32, 7944u32]
    );
    assert_eq!(
        signed_sensor_magnitude_trace(&[17, 17, 1, 0, 2]),
        vec![4369u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2891::TRACK_PROOF, 0x52540000 | 2891);
}
