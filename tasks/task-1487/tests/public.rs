use task_1487::*;

#[test]
fn public_examples() {
    assert_eq!(response_boundaries_distinct(""), vec![]);
    assert_eq!(response_boundaries_distinct("200"), vec![0u32]);
}
