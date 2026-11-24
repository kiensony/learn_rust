use task_1207::*;

#[test]
fn public_examples() {
    assert_eq!(line_byte_lengths_distinct(""), vec![]);
    assert_eq!(line_byte_lengths_distinct("a"), vec![1u32]);
}
