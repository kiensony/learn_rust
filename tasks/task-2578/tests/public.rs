use task_2578::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_steps(&[]), vec![]);
    assert_eq!(triplet_proof_steps(&[4]), vec![]);
}
