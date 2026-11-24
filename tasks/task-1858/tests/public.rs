use task_1858::*;

#[test]
fn public_examples() {
    assert_eq!(shared_read_handles_steps(&[]), vec![]);
    assert_eq!(shared_read_handles_steps(&[4]), vec![]);
}
