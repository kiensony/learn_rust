use task_1755::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_changes(&[]), vec![]);
    assert_eq!(build_multiplier_changes(&[4]), vec![12u32]);
}
