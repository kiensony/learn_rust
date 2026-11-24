use task_2288::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_steps(&[]), vec![]);
    assert_eq!(weighted_pressure_steps(&[4]), vec![]);
}
