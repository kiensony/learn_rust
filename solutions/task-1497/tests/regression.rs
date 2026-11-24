use task_1497::*;

#[test]
fn transfer_case() {
    assert_eq!(
        client_error_fixture_run_ledger("200 503 404 200 200 503 404 200"),
        vec![(404u32, 2usize)]
    );
}
