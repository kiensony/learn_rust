use task_2269::*;

#[test]
fn public_examples() {
    assert_eq!(hot_code_addresses_longest_run(&[]), 0);
    assert_eq!(hot_code_addresses_longest_run(&[4]), 0);
}
