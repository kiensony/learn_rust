use task_2262::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_total(&[]), 0);
    assert_eq!(hot_code_addresses_total(&[4]), 0);
}
