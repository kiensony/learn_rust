use task_2240::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_reverse(""), vec![]);
    assert_eq!(zero_copy_lines_reverse("a"), vec![1u32]);
}
