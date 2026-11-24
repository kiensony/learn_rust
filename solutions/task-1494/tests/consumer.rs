#[test]
fn malformed_status_does_not_discard_neighbors() {
    assert_eq!(
        task_1494::status_detail_peaks("503 invalid 200 301 404 500"),
        vec![3u32, 4u32]
    );
}
