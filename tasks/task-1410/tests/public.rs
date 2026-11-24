use task_1410::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_reverse(&[]), vec![]);
    assert_eq!(millisecond_remainder_reverse(&[4]), vec![68u32]);
}
