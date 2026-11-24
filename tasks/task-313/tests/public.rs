use task_313::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon(String::from("hi:kit")),
        "hi:kit"
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(String::from(":é:a")),
        "é:a"
    );
}
