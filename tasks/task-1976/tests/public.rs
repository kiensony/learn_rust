use task_1976::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_descending_steps(&[]), 0);
    assert_eq!(atomic_toggle_descending_steps(&[4]), 0);
}
