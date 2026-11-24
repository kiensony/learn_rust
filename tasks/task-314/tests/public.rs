use task_314::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon(String::from("hi:kit")), "hi");
    assert_eq!(prefix_before_final_mark_colon(String::from(":é:a")), ":é");
}
