use task_2567::*;

#[test]
fn public_examples() {
    assert_eq!(odd_element_proof_distinct(&[]), vec![]);
    assert_eq!(odd_element_proof_distinct(&[4]), vec![]);
}
