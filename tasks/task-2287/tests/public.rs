use task_2287::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_distinct(&[]), vec![]);
    assert_eq!(weighted_pressure_distinct(&[4]), vec![4u32]);
}
