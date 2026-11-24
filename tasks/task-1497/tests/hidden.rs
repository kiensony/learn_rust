use task_1497::*;

#[test]
fn edge_cases() {
    assert_eq!(client_error_fixture_run_ledger("200"), vec![]);
    assert_eq!(
        client_error_fixture_run_ledger("200 503 404 200"),
        vec![(404u32, 1usize)]
    );
    assert_eq!(client_error_fixture_run_ledger("xx 20 099 201 600"), vec![]);
    assert_eq!(
        client_error_fixture_run_ledger("500 500 204 301 404"),
        vec![(404u32, 1usize)]
    );
    assert_eq!(
        client_error_fixture_run_ledger(" 302 204 503 400 201 "),
        vec![(400u32, 1usize)]
    );
    assert_eq!(
        client_error_fixture_run_ledger("200 503 404 200"),
        vec![(404u32, 1usize)]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1497::TRACK_PROOF, 0x52540000 | 1497);
}
