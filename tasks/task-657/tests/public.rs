use task_657::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_colon(Event::Data("hi:kit")), Some(3));
    assert_eq!(closing_field_colon(Event::Data(":é:a")), Some(1));
}
