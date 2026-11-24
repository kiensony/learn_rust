use task_957::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon(vec!["hi:kit", ":é:a"]),
        vec![6usize, 4usize]
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(vec![":é:a", ""]),
        vec![4usize, 0usize]
    );
}
