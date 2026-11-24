use task_1210::*;

#[test]
fn public_examples() {
    assert_eq!(line_byte_lengths_reverse(""), vec![]);
    assert_eq!(line_byte_lengths_reverse("a"), vec![1u32]);
}
