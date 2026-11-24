use task_968::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_slash(vec!["path/1/12", "path/123"]),
        vec![1usize, 0usize]
    );
    assert_eq!(
        prefix_before_final_mark_slash(vec!["path/123", "hi/kit"]),
        vec![0usize, 2usize]
    );
}
