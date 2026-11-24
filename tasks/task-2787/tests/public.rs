use task_2787::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_distinct(""), vec![]);
    assert_eq!(parsed_defaults_distinct("a=1"), vec![]);
}
