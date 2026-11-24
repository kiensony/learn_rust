use task_712::*;

#[test]
fn public_examples() {
    assert_eq!(longest_field_with_last_tie_colon(Some("hi:kit")), None);
    assert_eq!(longest_field_with_last_tie_colon(Some("3:72")), Some(72));
}
