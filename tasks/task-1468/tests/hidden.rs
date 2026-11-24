use task_1468::*;

#[test]
fn edge_cases() {
    assert_eq!(client_error_fixture_steps("200 503 404 200"), vec![]);
    assert_eq!(client_error_fixture_steps("xx 20 099 201 600"), vec![]);
    assert_eq!(client_error_fixture_steps("500 500 204 301 404"), vec![]);
    assert_eq!(client_error_fixture_steps(" 302 204 503 400 201 "), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1468::TRACK_PROOF, 0x52540000 | 1468);
}
