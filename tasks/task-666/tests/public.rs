use task_666::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_slash(Event::Data("path/1/12")), Some(2));
    assert_eq!(closing_field_slash(Event::Data("path/123")), Some(3));
}
