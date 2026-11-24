use task_2580::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_reverse(&[]), vec![]);
    assert_eq!(triplet_proof_reverse(&[4]), vec![5u32]);
}
