use task_1453::*;

#[test]
fn edge_cases() {
    assert_eq!(
        http_status_fixture_bounds("200 503 404 200"),
        Some((2u32, 5u32))
    );
    assert_eq!(
        http_status_fixture_bounds("xx 20 099 201 600"),
        Some((2u32, 2u32))
    );
    assert_eq!(
        http_status_fixture_bounds("500 500 204 301 404"),
        Some((2u32, 5u32))
    );
    assert_eq!(
        http_status_fixture_bounds(" 302 204 503 400 201 "),
        Some((2u32, 5u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1453::TRACK_PROOF, 0x52540000 | 1453);
}
