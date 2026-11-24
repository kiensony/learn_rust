use task_2837::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_distinct(&[]), vec![]);
    assert_eq!(service_expansion_distinct(&[4]), vec![8u32]);
}
