use task_710::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_colon(Some("hi:kit")), None);
    assert_eq!(prefix_before_final_mark_colon(Some("3:72")), Some(3));
}
