use task_2759::*;

#[test]
fn public_examples() {
    assert_eq!(parsed_values_longest_run(""), 0);
    assert_eq!(parsed_values_longest_run("a=1"), 1);
}
