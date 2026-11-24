use task_991::*;

#[test]
fn public_examples() {
    assert_eq!(
        shortest_field_with_first_tie_equals(vec!["72;12", "猫=12;note"]),
        vec![2usize, 2usize]
    );
    assert_eq!(shortest_field_with_first_tie_equals(vec![]), vec![]);
}
