use task_2037::*;

#[test]
fn public_examples() {
    assert_eq!(poll_gate(0), (1, 0));
    assert_eq!(poll_gate(1), (2, 1));
}
