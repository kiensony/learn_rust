use task_970::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_slash(vec!["path/1/12", "path/123"]),
        vec![2usize, 3usize]
    );
    assert_eq!(
        longest_field_with_last_tie_slash(vec!["path/123", "hi/kit"]),
        vec![3usize, 3usize]
    );
}
