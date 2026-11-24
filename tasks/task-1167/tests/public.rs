use task_1167::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_slash("path/1/12", "backup"), "1");
    assert_eq!(
        prefix_before_final_mark_slash("path/123", "backup"),
        "backup"
    );
}
