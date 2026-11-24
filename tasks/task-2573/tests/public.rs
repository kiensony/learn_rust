use task_2573::*;

#[test]
fn public_examples() {
    assert_eq!(triplet_proof_bounds(&[]), None);
    assert_eq!(triplet_proof_bounds(&[4]), Some((5u32, 5u32)));
}
