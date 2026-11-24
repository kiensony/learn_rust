#[test]
fn malformed_status_does_not_discard_neighbors() {
    assert_eq!(
        task_1467::client_error_fixture_distinct("503 invalid 200 301 404 500"),
        vec![404u32]
    );
}
