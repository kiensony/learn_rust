use task_2282::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_total(&[]), 0);
    assert_eq!(weighted_pressure_total(&[4]), 4);
}
