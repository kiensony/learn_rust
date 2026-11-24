use task_2571::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_trace(&[]), vec![]);
    assert_eq!(triplet_proof_trace(&[4]), vec![5u32]);
}
