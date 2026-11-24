use task_2290::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_reverse(&[]), vec![]);
    assert_eq!(weighted_pressure_reverse(&[4]), vec![4u32]);
}
