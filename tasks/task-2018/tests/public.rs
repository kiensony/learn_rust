use task_2018::*;

#[test]
fn public_examples() {
    assert_eq!(polled_flags_steps(&[]), vec![]);
    assert_eq!(polled_flags_steps(&[4]), vec![]);
}
