use task_2568::*;

#[test]
fn public_examples() {
    assert_eq!(odd_element_proof_steps(&[]), vec![]);
    assert_eq!(odd_element_proof_steps(&[4]), vec![]);
}
