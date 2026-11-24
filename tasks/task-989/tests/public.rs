use task_989::*;

#[test]
fn public_examples() {
    assert_eq!(
        first_nonempty_field_equals(vec!["72;12", "猫=12;note"]),
        vec![2usize, 3usize]
    );
    assert_eq!(
        first_nonempty_field_equals(vec!["猫=12;note", "hi=kit"]),
        vec![3usize, 2usize]
    );
}
