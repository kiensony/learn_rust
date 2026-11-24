use task_2565::*;

#[test]
fn public_examples() {
    assert_eq!(odd_element_proof_changes(&[]), vec![]);
    assert_eq!(odd_element_proof_changes(&[4]), vec![]);
}
