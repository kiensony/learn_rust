use task_713::*;

#[test]
fn public_examples() {
    assert_eq!(shortest_field_with_first_tie_colon(Some("hi:kit")), None);
    assert_eq!(shortest_field_with_first_tie_colon(Some("3:72")), Some(3));
}
