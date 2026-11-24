use task_2281::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_trace(&[]), vec![]);
    assert_eq!(weighted_pressure_trace(&[4]), vec![4u32]);
}
