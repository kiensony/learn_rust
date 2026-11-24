use task_2840::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_reverse(&[]), vec![]);
    assert_eq!(service_expansion_reverse(&[4]), vec![8u32]);
}
