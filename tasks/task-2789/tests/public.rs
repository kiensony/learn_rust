use task_2789::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_defaults_longest_run(""), 0);
    assert_eq!(parsed_defaults_longest_run("a=1"), 0);
}
