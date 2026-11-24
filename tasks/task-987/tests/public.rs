use task_987::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_equals(vec!["72;12", "猫=12;note"]),
        vec![0usize, 2usize]
    );
    assert_eq!(
        remainder_after_first_mark_equals(vec!["猫=12;note", "hi=kit"]),
        vec![2usize, 3usize]
    );
}
