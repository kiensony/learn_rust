use task_2575::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_changes(&[]), vec![]);
    assert_eq!(triplet_proof_changes(&[4]), vec![5u32]);
}
