use task_2788::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_steps(""), vec![]);
    assert_eq!(parsed_defaults_steps("a=1"), vec![]);
}
