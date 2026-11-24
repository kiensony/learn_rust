use task_1464::*;

#[test]
fn edge_cases() {
    assert_eq!(client_error_fixture_peaks("200 503 404 200"), vec![404u32]);
    assert_eq!(client_error_fixture_peaks("xx 20 099 201 600"), vec![]);
    assert_eq!(
        client_error_fixture_peaks("500 500 204 301 404"),
        vec![404u32]
    );
    assert_eq!(
        client_error_fixture_peaks(" 302 204 503 400 201 "),
        vec![400u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1464::TRACK_PROOF, 0x52540000 | 1464);
}
