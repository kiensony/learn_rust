use task_1408::*;

#[test]
fn public_examples() {
    assert_eq!(millisecond_remainder_steps(&[]), vec![]);
    assert_eq!(millisecond_remainder_steps(&[4]), vec![]);
}
