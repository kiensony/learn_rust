use task_2783::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_bounds(""), None);
    assert_eq!(parsed_defaults_bounds("a=1"), None);
}
