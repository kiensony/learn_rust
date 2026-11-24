use task_2765::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_changes(""), vec![]);
    assert_eq!(parsed_key_lengths_changes("a=1"), vec![1u32]);
}
