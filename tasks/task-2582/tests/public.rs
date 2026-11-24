use task_2582::*;

#[test]
fn public_examples() {
    assert_eq!(stride_proof_total(&[]), 0);
    assert_eq!(stride_proof_total(&[4]), 0);
}
