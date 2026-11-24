use task_2015::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_changes(&[]), vec![]);
    assert_eq!(polled_flags_changes(&[4]), vec![1u32]);
}
