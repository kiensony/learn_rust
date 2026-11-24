use task_656::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_colon(Event::Data("hi:kit")), Some(2));
    assert_eq!(leading_field_colon(Event::Data(":é:a")), Some(0));
}
