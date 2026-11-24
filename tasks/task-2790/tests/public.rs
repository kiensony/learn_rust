use task_2790::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_reverse(""), vec![]);
    assert_eq!(parsed_defaults_reverse("a=1"), vec![]);
}
