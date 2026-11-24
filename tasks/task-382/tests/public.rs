use task_382::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe(&String::from("first\n123")),
        3
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(&String::from("header\n猫|123")),
        7
    );
}
