use task_1157::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon("hi:kit", "backup"), "hi");
    assert_eq!(prefix_before_final_mark_colon(":é:a", "backup"), ":é");
}
