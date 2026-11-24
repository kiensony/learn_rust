use task_1455::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_changes(""), vec![]);
    assert_eq!(http_status_fixture_changes("200"), vec![2u32]);
}
