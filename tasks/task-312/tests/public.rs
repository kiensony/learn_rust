use task_312::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_colon(String::from("hi:kit")),
        "hi:kit"
    );
    assert_eq!(closing_marks_removed_colon(String::from(":é:a")), ":é:a");
}
