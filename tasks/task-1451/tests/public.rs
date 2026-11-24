use task_1451::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_trace(""), vec![]);
    assert_eq!(http_status_fixture_trace("200"), vec![2u32]);
}
