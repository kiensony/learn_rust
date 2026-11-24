use task_2267::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_distinct(&[]), vec![]);
    assert_eq!(hot_code_addresses_distinct(&[4]), vec![]);
}
