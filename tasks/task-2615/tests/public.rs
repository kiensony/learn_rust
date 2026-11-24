use task_2615::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_changes(&[]), vec![]);
    assert_eq!(embedded_flags_changes(&[4]), vec![4u32]);
}
