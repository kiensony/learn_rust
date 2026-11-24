use task_963::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_slash(vec!["path/1/12", "path/123"]),
        vec![2usize, 3usize]
    );
    assert_eq!(
        closing_field_slash(vec!["path/123", "hi/kit"]),
        vec![3usize, 3usize]
    );
}
