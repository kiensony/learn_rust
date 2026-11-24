use task_361::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon(&String::from("hi:kit")), 6);
    assert_eq!(closing_marks_removed_colon(&String::from(":é:a")), 5);
}
