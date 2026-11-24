use task_1751::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_trace(&[]), vec![]);
    assert_eq!(build_multiplier_trace(&[4]), vec![12u32]);
}
