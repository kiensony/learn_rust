use task_1474::*;

#[test]
fn public_examples() {
    assert_eq!(retry_fixture_peaks(""), vec![]);
    assert_eq!(retry_fixture_peaks("200"), vec![]);
}
