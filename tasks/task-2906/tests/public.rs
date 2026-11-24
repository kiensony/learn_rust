use task_2906::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_descending_steps(&[]), 0);
    assert_eq!(latency_increases_descending_steps(&[4]), 0);
}
