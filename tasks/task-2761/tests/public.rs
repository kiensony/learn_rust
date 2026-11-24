use task_2761::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_trace(""), vec![]);
    assert_eq!(parsed_key_lengths_trace("a=1"), vec![1u32]);
}
