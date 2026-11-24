use task_1457::*;

#[test]
fn edge_cases() {
    assert_eq!(
        http_status_fixture_distinct("200 503 404 200"),
        vec![2u32, 4u32, 5u32]
    );
    assert_eq!(
        http_status_fixture_distinct("xx 20 099 201 600"),
        vec![2u32]
    );
    assert_eq!(
        http_status_fixture_distinct("500 500 204 301 404"),
        vec![2u32, 3u32, 4u32, 5u32]
    );
    assert_eq!(
        http_status_fixture_distinct(" 302 204 503 400 201 "),
        vec![2u32, 3u32, 4u32, 5u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1457::TRACK_PROOF, 0x52540000 | 1457);
}
