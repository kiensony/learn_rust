use task_563::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon("hi:kit"), "hi");
    assert_eq!(prefix_before_final_mark_colon(":é:a"), ":é");
}
