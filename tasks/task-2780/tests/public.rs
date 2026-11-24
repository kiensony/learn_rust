use task_2780::*;

#[test]
fn public_examples() {
    assert_eq!(repeated_keys_reverse(""), vec![]);
    assert_eq!(repeated_keys_reverse("a=1"), vec![]);
}
