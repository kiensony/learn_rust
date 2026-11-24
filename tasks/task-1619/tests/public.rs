use task_1619::*;

#[test]
fn public_examples() {
    assert_eq!(saturation_property_longest_run(&[]), 0);
    assert_eq!(saturation_property_longest_run(&[4]), 1);
}
