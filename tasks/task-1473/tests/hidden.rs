use task_1473::*;

#[test]
fn edge_cases() {
    assert_eq!(
        retry_fixture_bounds("200 503 404 200"),
        Some((503u32, 503u32))
    );
    assert_eq!(retry_fixture_bounds("xx 20 099 201 600"), None);
    assert_eq!(
        retry_fixture_bounds("500 500 204 301 404"),
        Some((500u32, 500u32))
    );
    assert_eq!(
        retry_fixture_bounds(" 302 204 503 400 201 "),
        Some((503u32, 503u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1473::TRACK_PROOF, 0x52540000 | 1473);
}
