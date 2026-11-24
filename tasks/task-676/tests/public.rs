use task_676::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_pipe(Event::Data("first\n123")), Some(3));
    assert_eq!(closing_field_pipe(Event::Data("|é|a")), Some(1));
}
