use task_2770::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_reverse(""), vec![]);
    assert_eq!(parsed_key_lengths_reverse("a=1"), vec![1u32]);
}
