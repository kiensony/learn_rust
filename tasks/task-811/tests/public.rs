use task_811::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_colon("hi:kit"), Ok(3));
    assert_eq!(longest_field_with_last_tie_colon(":é:a"), Ok(2));
}
