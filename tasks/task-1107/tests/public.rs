use task_1107::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon(&"hi:kit"), 2);
    assert_eq!(prefix_before_final_mark_colon(&String::from(":é:a")), 3);
}
