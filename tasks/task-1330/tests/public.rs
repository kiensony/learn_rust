use task_1330::*;

#[test]
fn public_examples() {
    assert_eq!(flag_fields_reverse(""), vec![]);
    assert_eq!(flag_fields_reverse("1"), vec![1u32]);
}
