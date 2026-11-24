use task_986::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_equals(vec!["72;12", "猫=12;note"]),
        vec![2usize, 6usize]
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(vec!["猫=12;note", "hi=kit"]),
        vec![6usize, 6usize]
    );
}
