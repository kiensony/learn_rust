use task_2588::*;

#[test]
fn public_examples() {
    assert_eq!(stride_proof_steps(&[]), vec![]);
    assert_eq!(stride_proof_steps(&[4]), vec![]);
}
