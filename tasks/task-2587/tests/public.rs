use task_2587::*;

#[test]
fn public_examples() {
    assert_eq!(stride_proof_distinct(&[]), vec![]);
    assert_eq!(stride_proof_distinct(&[4]), vec![]);
}
