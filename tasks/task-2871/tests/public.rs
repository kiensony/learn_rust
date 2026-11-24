use task_2871::*;

#[test]
fn public_examples() {
    assert_eq!(flag_register_trace(&[]), vec![]);
    assert_eq!(flag_register_trace(&[4]), vec![]);
}
