use task_2268::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_steps(&[]), vec![]);
    assert_eq!(hot_code_addresses_steps(&[4]), vec![]);
}
