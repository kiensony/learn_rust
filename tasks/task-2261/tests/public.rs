use task_2261::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_trace(&[]), vec![]);
    assert_eq!(hot_code_addresses_trace(&[4]), vec![]);
}
