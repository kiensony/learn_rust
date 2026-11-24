use task_1205::*;

#[test]
fn public_examples() {
    assert_eq!(line_byte_lengths_changes(""), vec![]);
    assert_eq!(line_byte_lengths_changes("a"), vec![1u32]);
}
