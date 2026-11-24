use task_1204::*;

#[test]
fn public_examples() {
    assert_eq!(line_byte_lengths_peaks(""), vec![]);
    assert_eq!(line_byte_lengths_peaks("a"), vec![1u32]);
}
