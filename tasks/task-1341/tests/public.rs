use task_1341::*;

#[test]
fn public_examples() {
    assert_eq!(packed_booleans_trace(""), vec![]);
    assert_eq!(packed_booleans_trace("1"), vec![1u32]);
}
