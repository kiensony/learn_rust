use task_1418::*;

#[test]
fn public_examples() {
    assert_eq!(elapsed_seconds_steps(&[]), vec![]);
    assert_eq!(elapsed_seconds_steps(&[4]), vec![]);
}
