use task_2835::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_changes(&[]), vec![]);
    assert_eq!(service_expansion_changes(&[4]), vec![8u32]);
}
