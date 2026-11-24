use task_1607::*;

#[test]
fn public_examples() {
    assert_eq!(parity_property_distinct(&[]), vec![]);
    assert_eq!(parity_property_distinct(&[4]), vec![0u32]);
}
