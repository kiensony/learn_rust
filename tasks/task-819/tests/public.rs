use task_819::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_slash("path/1/12"), Ok(1));
    assert_eq!(
        prefix_before_final_mark_slash("path/123"),
        Err(FieldError::Missing)
    );
}
