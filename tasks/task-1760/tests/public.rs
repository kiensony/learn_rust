use task_1760::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_reverse(&[]), vec![]);
    assert_eq!(build_multiplier_reverse(&[4]), vec![12u32]);
}
