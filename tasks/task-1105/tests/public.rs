use task_1105::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_colon(&"hi:kit"), 6);
    assert_eq!(marks_removed_at_both_ends_colon(&String::from(":é:a")), 4);
}
