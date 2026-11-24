use task_2285::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_changes(&[]), vec![]);
    assert_eq!(weighted_pressure_changes(&[4]), vec![4u32]);
}
