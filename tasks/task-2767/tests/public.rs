use task_2767::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_distinct(""), vec![]);
    assert_eq!(parsed_key_lengths_distinct("a=1"), vec![1u32]);
}
