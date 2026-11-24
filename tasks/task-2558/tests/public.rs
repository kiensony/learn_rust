use task_2558::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_steps(&[]), vec![]);
    assert_eq!(split_proof_steps(&[4]), vec![]);
}
