use task_1497::*;

#[test]
fn public_examples() {
    assert_eq!(
        client_error_fixture_run_ledger("200 503 404 200"),
        vec![(404u32, 1usize)]
    );
    assert_eq!(client_error_fixture_run_ledger(""), vec![]);
}
