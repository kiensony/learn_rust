use task_1480::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_reverse(""), vec![]);
    assert_eq!(retry_fixture_reverse("200"), vec![]);
}
