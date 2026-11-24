use task_1868::*;

#[test]
fn public_examples() {
    assert_eq!(weak_read_handles_steps(&[]), vec![]);
    assert_eq!(weak_read_handles_steps(&[4]), vec![]);
}
