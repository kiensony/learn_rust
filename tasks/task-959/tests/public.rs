use task_959::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_colon(vec!["hi:kit", ":é:a"]),
        vec![2usize, 2usize]
    );
    assert_eq!(
        first_nonempty_field_colon(vec![":é:a", ""]),
        vec![2usize, 0usize]
    );
}
