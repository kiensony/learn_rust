use task_2782::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_total(""), 0);
    assert_eq!(parsed_defaults_total("a=1"), 0);
}
