use task_2559::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_longest_run(&[]), 0);
    assert_eq!(split_proof_longest_run(&[4]), 1);
}
