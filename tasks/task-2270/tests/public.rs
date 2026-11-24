use task_2270::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_reverse(&[]), vec![]);
    assert_eq!(hot_code_addresses_reverse(&[4]), vec![]);
}
