use task_1310::*;

#[test]
fn public_examples() {
    assert_eq!(decimal_fields_reverse(""), vec![]);
    assert_eq!(decimal_fields_reverse("1"), vec![1u32]);
}
