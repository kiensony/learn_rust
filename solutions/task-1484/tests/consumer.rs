#[test]
fn malformed_status_does_not_discard_neighbors() {
    assert_eq!(
        task_1484::response_boundaries_peaks("503 invalid 200 301 404 500"),
        vec![1u32, 2u32]
    );
}
