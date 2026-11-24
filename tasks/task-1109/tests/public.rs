use task_1109::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_colon(&"hi:kit"), 3);
    assert_eq!(longest_field_with_last_tie_colon(&String::from(":é:a")), 2);
}
