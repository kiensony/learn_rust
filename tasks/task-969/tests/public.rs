use task_969::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_slash(vec!["path/1/12", "path/123"]),
        vec![1usize, 3usize]
    );
    assert_eq!(
        first_nonempty_field_slash(vec!["path/123", "hi/kit"]),
        vec![3usize, 2usize]
    );
}
