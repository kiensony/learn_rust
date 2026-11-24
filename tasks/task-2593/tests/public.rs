use task_2593::*;

#[test]
fn public_examples() {
    assert_eq!(interior_stride_proof_bounds(&[]), None);
    assert_eq!(interior_stride_proof_bounds(&[4]), None);
}
