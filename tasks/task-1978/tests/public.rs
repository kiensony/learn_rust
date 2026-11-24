use task_1978::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_steps(&[]), vec![]);
    assert_eq!(atomic_toggle_steps(&[4]), vec![]);
}
