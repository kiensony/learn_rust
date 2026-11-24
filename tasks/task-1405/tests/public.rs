use task_1405::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_changes(&[]), vec![]);
    assert_eq!(millisecond_remainder_changes(&[4]), vec![68u32]);
}
