use task_2785::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_changes(""), vec![]);
    assert_eq!(parsed_defaults_changes("a=1"), vec![]);
}
