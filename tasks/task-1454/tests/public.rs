use task_1454::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_peaks(""), vec![]);
    assert_eq!(http_status_fixture_peaks("200"), vec![2u32]);
}
