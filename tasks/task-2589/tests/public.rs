use task_2589::*;

#[test]
fn public_examples() {
    assert_eq!(stride_proof_longest_run(&[]), 0);
    assert_eq!(stride_proof_longest_run(&[4]), 0);
}
