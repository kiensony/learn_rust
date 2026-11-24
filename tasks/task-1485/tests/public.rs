use task_1485::*;

#[test]
fn public_examples() {
    assert_eq!(response_boundaries_changes(""), vec![]);
    assert_eq!(response_boundaries_changes("200"), vec![0u32]);
}
