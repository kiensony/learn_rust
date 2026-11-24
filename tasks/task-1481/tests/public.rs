use task_1481::*;

#[test]
fn public_examples() {
    assert_eq!(response_boundaries_trace(""), vec![]);
    assert_eq!(response_boundaries_trace("200"), vec![0u32]);
}
