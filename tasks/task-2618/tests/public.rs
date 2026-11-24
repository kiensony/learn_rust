use task_2618::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_steps(&[]), vec![]);
    assert_eq!(embedded_flags_steps(&[4]), vec![]);
}
