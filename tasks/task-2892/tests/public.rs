use task_2892::*;

#[test]
fn public_examples() {
    assert_eq!(signed_sensor_magnitude_total(&[]), 0);
    assert_eq!(signed_sensor_magnitude_total(&[4]), 0);
}
