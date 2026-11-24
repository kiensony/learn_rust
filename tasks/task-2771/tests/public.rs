use task_2771::*;

#[test]
fn public_examples() {
    assert_eq!(repeated_keys_trace(""), vec![]);
    assert_eq!(repeated_keys_trace("a=1"), vec![]);
}
