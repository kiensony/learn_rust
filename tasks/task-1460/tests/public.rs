use task_1460::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_reverse(""), vec![]);
    assert_eq!(http_status_fixture_reverse("200"), vec![2u32]);
}
