use task_2551::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_trace(&[]), vec![]);
    assert_eq!(split_proof_trace(&[4]), vec![5u32]);
}
