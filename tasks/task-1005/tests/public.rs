use task_1005::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_colon(["hi:kit", ":é:a"]), [6, 5]);
    assert_eq!(
        closing_marks_removed_colon([String::from(":é:a"), String::from("")]),
        [5, 0]
    );
}
