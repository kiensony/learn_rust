use task_386::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_pipe(&String::from("first\n123")),
        3
    );
    assert_eq!(longest_field_with_last_tie_pipe(&String::from("|é|a")), 2);
}
