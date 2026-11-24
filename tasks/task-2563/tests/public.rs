use task_2563::*;

#[test]
fn public_examples() {
    assert_eq!(odd_element_proof_bounds(&[]), None);
    assert_eq!(odd_element_proof_bounds(&[4]), None);
}
