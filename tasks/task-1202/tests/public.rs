use task_1202::*;

#[test]
fn public_examples() {
    assert_eq!(line_byte_lengths_total(""), 0);
    assert_eq!(line_byte_lengths_total("a"), 1);
}
