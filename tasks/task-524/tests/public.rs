use task_524::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_slash("path/1/12"), 1);
    assert_eq!(prefix_before_final_mark_slash("path/123"), 0);
}
