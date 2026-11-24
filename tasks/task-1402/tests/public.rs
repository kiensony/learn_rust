use task_1402::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_total(&[]), 0);
    assert_eq!(millisecond_remainder_total(&[4]), 68);
}
