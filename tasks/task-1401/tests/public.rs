use task_1401::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_trace(&[]), vec![]);
    assert_eq!(millisecond_remainder_trace(&[4]), vec![68u32]);
}
