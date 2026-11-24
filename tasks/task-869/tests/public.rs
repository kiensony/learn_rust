use task_869::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_slash("path/1/12"), 3);
    assert_eq!(prefix_before_final_mark_slash("path/123"), 2);
}
