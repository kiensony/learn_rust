use task_1457::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_distinct(""), vec![]);
    assert_eq!(http_status_fixture_distinct("200"), vec![2u32]);
}
