use task_2577::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_distinct(&[]), vec![]);
    assert_eq!(triplet_proof_distinct(&[4]), vec![5u32]);
}
