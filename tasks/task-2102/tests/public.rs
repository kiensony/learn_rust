use task_2102::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_total(&[]), 0);
    assert_eq!(sorted_keys_total(&[4]), 4);
}
