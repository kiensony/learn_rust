use task_720::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_slash(Some("path/1/12")), Some(1));
    assert_eq!(prefix_before_final_mark_slash(Some("path/123")), None);
}
