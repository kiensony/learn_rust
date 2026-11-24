use task_2572::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_total(&[]), 0);
    assert_eq!(triplet_proof_total(&[4]), 5);
}
