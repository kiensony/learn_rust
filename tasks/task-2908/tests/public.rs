use task_2908::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_steps(&[]), vec![]);
    assert_eq!(latency_increases_steps(&[4]), vec![]);
}
