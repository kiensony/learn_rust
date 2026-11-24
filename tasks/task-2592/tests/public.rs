use task_2592::*;

#[test]
fn public_examples() {
    assert_eq!(interior_stride_proof_total(&[]), 0);
    assert_eq!(interior_stride_proof_total(&[4]), 0);
}
