use task_2579::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_longest_run(&[]), 0);
    assert_eq!(triplet_proof_longest_run(&[4]), 1);
}
