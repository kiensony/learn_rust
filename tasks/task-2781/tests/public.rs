use task_2781::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_trace(""), vec![]);
    assert_eq!(parsed_defaults_trace("a=1"), vec![]);
}
