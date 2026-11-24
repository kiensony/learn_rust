use task_1129::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_pipe(&"first\n123"), 3);
    assert_eq!(longest_field_with_last_tie_pipe(&String::from("|é|a")), 2);
}
