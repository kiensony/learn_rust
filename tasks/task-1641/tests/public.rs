use task_1641::*;

#[test]
fn public_examples() {
    assert_eq!(shift_property_trace(&[]), vec![]);
    assert_eq!(shift_property_trace(&[4]), vec![0u32]);
}
