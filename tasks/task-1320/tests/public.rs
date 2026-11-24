use task_1320::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_fields_reverse(""), vec![]);
    assert_eq!(nibble_fields_reverse("1"), vec![0u32]);
}
