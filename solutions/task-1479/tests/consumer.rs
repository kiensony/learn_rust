#[test]
fn malformed_status_does_not_discard_neighbors() {
    assert_eq!(
        task_1479::retry_fixture_longest_run("503 invalid 200 301 404 500"),
        1
    );
}
