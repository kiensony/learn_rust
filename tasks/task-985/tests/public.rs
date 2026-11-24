use task_985::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_equals(vec!["72;12", "猫=12;note"]),
        vec![2usize, 6usize]
    );
    assert_eq!(
        closing_marks_removed_equals(vec!["猫=12;note", "hi=kit"]),
        vec![6usize, 6usize]
    );
}
