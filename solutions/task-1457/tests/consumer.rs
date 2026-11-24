#[test]
fn malformed_status_does_not_discard_neighbors() {
    assert_eq!(
        task_1457::http_status_fixture_distinct("503 invalid 200 301 404 500"),
        vec![2u32, 3u32, 4u32, 5u32]
    );
}
