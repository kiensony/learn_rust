use task_2552::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_total(&[]), 0);
    assert_eq!(split_proof_total(&[4]), 5);
}
