use task_2265::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_changes(&[]), vec![]);
    assert_eq!(hot_code_addresses_changes(&[4]), vec![]);
}
