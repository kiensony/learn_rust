use task_740::*;

#[test]
fn public_examples() {
    assert_eq!(prefix_before_final_mark_equals(Some("72;12")), None);
    assert_eq!(prefix_before_final_mark_equals(Some("3=72")), Some(3));
}
