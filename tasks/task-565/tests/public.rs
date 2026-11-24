use task_565::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_colon("hi:kit"), "kit");
    assert_eq!(longest_field_with_last_tie_colon(":é:a"), "é");
}
