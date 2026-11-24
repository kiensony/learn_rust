use task_2768::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_key_lengths_steps(""), vec![]);
    assert_eq!(parsed_key_lengths_steps("a=1"), vec![]);
}
