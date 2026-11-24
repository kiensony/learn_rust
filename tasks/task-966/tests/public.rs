use task_966::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_slash(vec!["path/1/12", "path/123"]),
        vec![4usize, 3usize]
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(vec!["path/123", "hi/kit"]),
        vec![3usize, 6usize]
    );
}
