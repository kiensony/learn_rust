use task_2557::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_distinct(&[]), vec![]);
    assert_eq!(split_proof_distinct(&[4]), vec![5u32]);
}
