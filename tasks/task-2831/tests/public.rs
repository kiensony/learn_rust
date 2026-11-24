use task_2831::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_trace(&[]), vec![]);
    assert_eq!(service_expansion_trace(&[4]), vec![8u32]);
}
