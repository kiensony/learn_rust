use task_1260::*;

#[test]
fn public_examples() {
    assert_eq!(command_arguments_reverse(""), vec![]);
    assert_eq!(command_arguments_reverse("only"), vec![4u32]);
}
