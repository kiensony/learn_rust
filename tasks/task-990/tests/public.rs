use task_990::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_equals(vec!["72;12", "猫=12;note"]),
        vec![2usize, 3usize]
    );
    assert_eq!(
        longest_field_with_last_tie_equals(vec!["猫=12;note", "hi=kit"]),
        vec![3usize, 3usize]
    );
}
