use task_861::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_colon("hi:kit"), 5);
    assert_eq!(longest_field_with_last_tie_colon(":é:a"), 4);
}
