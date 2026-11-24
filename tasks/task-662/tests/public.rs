use task_662::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_colon(Event::Data("hi:kit")), Some(2));
    assert_eq!(first_nonempty_field_colon(Event::Data("")), Some(0));
}
