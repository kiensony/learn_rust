use task_1460::*;

#[test]
fn edge_cases() {
    assert_eq!(
        http_status_fixture_reverse("200 503 404 200"),
        vec![2u32, 4u32, 5u32, 2u32]
    );
    assert_eq!(http_status_fixture_reverse("xx 20 099 201 600"), vec![2u32]);
    assert_eq!(
        http_status_fixture_reverse("500 500 204 301 404"),
        vec![4u32, 3u32, 2u32, 5u32, 5u32]
    );
    assert_eq!(
        http_status_fixture_reverse(" 302 204 503 400 201 "),
        vec![2u32, 4u32, 5u32, 2u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1460::TRACK_PROOF, 0x52540000 | 1460);
}
