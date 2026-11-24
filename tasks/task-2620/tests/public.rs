use task_2620::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_reverse(&[]), vec![]);
    assert_eq!(embedded_flags_reverse(&[4]), vec![4u32]);
}
