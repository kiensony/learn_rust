use task_2595::*;

#[test]
fn public_examples() {
    assert_eq!(interior_stride_proof_changes(&[]), vec![]);
    assert_eq!(interior_stride_proof_changes(&[4]), vec![]);
}
