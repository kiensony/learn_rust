use task_2555::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_changes(&[]), vec![]);
    assert_eq!(split_proof_changes(&[4]), vec![5u32]);
}
