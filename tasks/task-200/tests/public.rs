use task_200::*;

#[test]
fn public_examples() {
    assert_eq!(admission_gate_10(65535, 4), false);
    assert_eq!(admission_gate_10(13, 14), true);
}
