use task_1407::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_distinct(&[]), vec![]);
    assert_eq!(millisecond_remainder_distinct(&[4]), vec![68u32]);
}
