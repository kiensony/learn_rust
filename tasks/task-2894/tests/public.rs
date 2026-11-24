use task_2894::*;

#[test]
fn public_examples() {
    assert_eq!(signed_sensor_magnitude_peaks(&[]), vec![]);
    assert_eq!(signed_sensor_magnitude_peaks(&[4]), vec![]);
}
