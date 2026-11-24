use task_956::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_colon(vec!["hi:kit", ":é:a"]),
        vec![6usize, 5usize]
    );
    assert_eq!(
        closing_marks_removed_colon(vec![":é:a", ""]),
        vec![5usize, 0usize]
    );
}
