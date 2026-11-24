use task_961::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_colon(vec!["hi:kit", ":é:a"]),
        vec![2usize, 0usize]
    );
    assert_eq!(
        shortest_field_with_first_tie_colon(vec![":é:a", ""]),
        vec![0usize, 0usize]
    );
}
