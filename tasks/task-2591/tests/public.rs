use task_2591::*;

#[test]
fn public_examples() {
    assert_eq!(interior_stride_proof_trace(&[]), vec![]);
    assert_eq!(interior_stride_proof_trace(&[4]), vec![]);
}
