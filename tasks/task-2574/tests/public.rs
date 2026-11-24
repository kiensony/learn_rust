use task_2574::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_peaks(&[]), vec![]);
    assert_eq!(triplet_proof_peaks(&[4]), vec![5u32]);
}
