use task_967::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_slash(vec!["path/1/12", "path/123"]),
        vec![2usize, 0usize]
    );
    assert_eq!(
        remainder_after_first_mark_slash(vec!["path/123", "hi/kit"]),
        vec![0usize, 3usize]
    );
}
