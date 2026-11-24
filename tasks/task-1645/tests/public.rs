use task_1645::*;

#[test]
fn public_examples() {
    assert_eq!(shift_property_changes(&[]), vec![]);
    assert_eq!(shift_property_changes(&[4]), vec![0u32]);
}
