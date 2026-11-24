use task_1609::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_longest_run(&[]), 0);
    assert_eq!(parity_property_longest_run(&[4]), 1);
}
