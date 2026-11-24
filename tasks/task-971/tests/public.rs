use task_971::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_slash(vec!["path/1/12", "path/123"]),
        vec![1usize, 3usize]
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(vec!["path/123", "hi/kit"]),
        vec![3usize, 2usize]
    );
}
