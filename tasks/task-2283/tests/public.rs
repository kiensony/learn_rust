use task_2283::*;

#[test]
fn public_examples() {
    assert_eq!(weighted_pressure_bounds(&[]), None);
    assert_eq!(weighted_pressure_bounds(&[4]), Some((4u32, 4u32)));
}
