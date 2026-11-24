use task_2554::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_peaks(&[]), vec![]);
    assert_eq!(split_proof_peaks(&[4]), vec![5u32]);
}
