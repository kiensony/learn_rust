use task_2895::*;

#[test]
fn public_examples() {
    assert_eq!(signed_sensor_magnitude_changes(&[]), vec![]);
    assert_eq!(signed_sensor_magnitude_changes(&[4]), vec![]);
}
