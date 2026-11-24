use task_2893::*;

#[test]
fn public_examples() {
    assert_eq!(signed_sensor_magnitude_bounds(&[]), None);
    assert_eq!(signed_sensor_magnitude_bounds(&[4]), None);
}
