use task_1484::*;

#[test]
fn public_examples() {
    assert_eq!(response_boundaries_peaks(""), vec![]);
    assert_eq!(response_boundaries_peaks("200"), vec![0u32]);
}
