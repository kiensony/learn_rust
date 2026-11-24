use task_1452::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_total(""), 0);
    assert_eq!(http_status_fixture_total("200"), 2);
}
