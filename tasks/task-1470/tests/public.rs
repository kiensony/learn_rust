use task_1470::*;

#[test]
fn public_examples() {
    assert_eq!(client_error_fixture_reverse(""), vec![]);
    assert_eq!(client_error_fixture_reverse("200"), vec![]);
}
