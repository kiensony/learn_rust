use task_2263::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_bounds(&[]), None);
    assert_eq!(hot_code_addresses_bounds(&[4]), None);
}
