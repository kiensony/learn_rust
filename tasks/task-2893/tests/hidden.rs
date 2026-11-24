use task_2893::*;

#[test]
fn edge_cases() {
    assert_eq!(
        signed_sensor_magnitude_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((521u32, 2050u32))
    );
    assert_eq!(
        signed_sensor_magnitude_bounds(&[0, 0, 0]),
        Some((0u32, 0u32))
    );
    assert_eq!(
        signed_sensor_magnitude_bounds(&[255, 1, 255, 2]),
        Some((511u32, 767u32))
    );
    assert_eq!(
        signed_sensor_magnitude_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((1539u32, 7944u32))
    );
    assert_eq!(
        signed_sensor_magnitude_bounds(&[17, 17, 1, 0, 2]),
        Some((1u32, 4369u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2893::TRACK_PROOF, 0x52540000 | 2893);
}
