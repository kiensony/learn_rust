use task_2760::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_reverse(""), vec![]);
    assert_eq!(parsed_values_reverse("a=1"), vec![1u32]);
}
