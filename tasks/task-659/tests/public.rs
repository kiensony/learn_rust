use task_659::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon(Event::Data("hi:kit")), Some(6));
    assert_eq!(closing_marks_removed_colon(Event::Data(":é:a")), Some(5));
}
