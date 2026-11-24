use task_2583::*;

#[test]
fn public_examples() {
    assert_eq!(stride_proof_bounds(&[]), None);
    assert_eq!(stride_proof_bounds(&[4]), None);
}
