use task_983::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_field_equals(vec!["72;12", "猫=12;note"]),
        vec![2usize, 2usize]
    );
    assert_eq!(
        closing_field_equals(vec!["猫=12;note", "hi=kit"]),
        vec![2usize, 3usize]
    );
}
