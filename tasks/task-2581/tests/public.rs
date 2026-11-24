use task_2581::*;

#[test]
fn public_examples() {
    assert_eq!(stride_proof_trace(&[]), vec![]);
    assert_eq!(stride_proof_trace(&[4]), vec![]);
}
