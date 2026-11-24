use task_2891::*;

#[test]
fn public_examples() {
    assert_eq!(signed_sensor_magnitude_trace(&[]), vec![]);
    assert_eq!(signed_sensor_magnitude_trace(&[4]), vec![]);
}
