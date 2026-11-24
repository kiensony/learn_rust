use task_2109::*;

#[test]
fn public_examples() {
    assert_eq!(sorted_keys_longest_run(&[]), 0);
    assert_eq!(sorted_keys_longest_run(&[4]), 1);
}
