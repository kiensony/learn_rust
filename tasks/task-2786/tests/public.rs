use task_2786::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_descending_steps(""), 0);
    assert_eq!(parsed_defaults_descending_steps("a=1"), 0);
}
