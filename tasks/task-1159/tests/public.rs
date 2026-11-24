use task_1159::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_colon("hi:kit", "backup"), "kit");
    assert_eq!(longest_field_with_last_tie_colon(":é:a", "backup"), "é");
}
