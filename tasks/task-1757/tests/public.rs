use task_1757::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_distinct(&[]), vec![]);
    assert_eq!(build_multiplier_distinct(&[4]), vec![12u32]);
}
