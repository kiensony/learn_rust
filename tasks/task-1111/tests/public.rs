use task_1111::*;

#[test]
fn public_examples() {
    assert_eq!(sample_all(&[&Constant(2), &Negate(3)]), vec![2, -3]);
    assert_eq!(sample_all(&[&Negate(-2)]), vec![2]);
}
