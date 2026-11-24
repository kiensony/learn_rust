use task_1270::*;

#[test]
fn public_examples() {
    assert_eq!(positional_options_reverse(""), vec![]);
    assert_eq!(positional_options_reverse("only"), vec![4u32]);
}
