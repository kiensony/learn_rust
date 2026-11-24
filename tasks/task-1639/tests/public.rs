use task_1639::*;

#[test]
fn public_examples() {
    assert_eq!(clamp_property_longest_run(&[]), 0);
    assert_eq!(clamp_property_longest_run(&[4]), 1);
}
