use task_2838::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_steps(&[]), vec![]);
    assert_eq!(service_expansion_steps(&[4]), vec![]);
}
