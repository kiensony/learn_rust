use task_2778::*;

#[test]
fn public_examples() {
    assert_eq!(repeated_keys_steps(""), vec![]);
    assert_eq!(repeated_keys_steps("a=1"), vec![]);
}
