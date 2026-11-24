use task_1456::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_descending_steps(""), 0);
    assert_eq!(http_status_fixture_descending_steps("200"), 0);
}
