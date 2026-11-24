use task_960::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_colon(vec!["hi:kit", ":é:a"]),
        vec![3usize, 2usize]
    );
    assert_eq!(
        longest_field_with_last_tie_colon(vec![":é:a", ""]),
        vec![2usize, 0usize]
    );
}
