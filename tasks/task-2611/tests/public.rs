use task_2611::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_trace(&[]), vec![]);
    assert_eq!(embedded_flags_trace(&[4]), vec![4u32]);
}
