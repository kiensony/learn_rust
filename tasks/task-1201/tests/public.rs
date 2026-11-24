use task_1201::*;

#[test]
fn public_examples() {
    assert_eq!(line_byte_lengths_trace(""), vec![]);
    assert_eq!(line_byte_lengths_trace("a"), vec![1u32]);
}
