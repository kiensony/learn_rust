use task_2576::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_descending_steps(&[]), 0);
    assert_eq!(triplet_proof_descending_steps(&[4]), 0);
}
